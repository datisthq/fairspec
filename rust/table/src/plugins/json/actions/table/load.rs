use std::io::{Cursor, Read};

use fairspec_dataset::{load_file, load_file_stream, prefetch_files};
use fairspec_metadata::{
    Error, FileDialect, PathOr, Resource, Result, RowType, get_supported_file_dialect,
    resolve_table_schema,
};
use polars::prelude::*;
use serde_json::Value;

use crate::actions::data::file_dialect::{get_header_labels_from_rows, get_records_from_rows};
use crate::actions::table::file_dialect::{assert_part_headers, concat_diagonal_tables};
use crate::actions::table::normalize::normalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::models::data::{DataRecord, DataRow};
use crate::models::table::{LoadTableOptions, Table};
use crate::plugins::json::actions::buffer::decode::decode_json_buffer;
use crate::plugins::json::actions::file_dialect::infer::infer_json_file_dialect;
use crate::plugins::json::helpers::pointer::parse_json_pointer;

/// Loads a JSON or JSONL file as a table; `None` when its rows are not an array.
pub fn load_json_table(resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
    let Some(mut file_dialect) = get_supported_file_dialect(resource, &["json", "jsonl"])? else {
        return Err(Error::Load("Resource data is not compatible".to_string()));
    };

    let is_lines = file_dialect.format() == Some("jsonl");
    let max_bytes = if is_lines {
        options.preview_bytes
    } else {
        None
    };
    let paths = prefetch_files(
        resource,
        max_bytes,
        options.infer_table_schema_options.concurrency,
    )?;
    let Some(first_path) = paths.first() else {
        return Err(Error::Load("Resource data is not defined".to_string()));
    };

    if resource.file_dialect.is_none() {
        let resource = Resource {
            data: Some(Value::String(first_path.clone())),
            file_dialect: Some(PathOr::Value(file_dialect.clone())),
            ..Resource::default()
        };
        if let Some(inferred) = infer_json_file_dialect(&resource, None)? {
            file_dialect = inferred;
        }
    }

    let is_lines = file_dialect.format() == Some("jsonl");
    let is_default = get_is_default_dialect(&file_dialect);

    let mut tables = Vec::new();
    let mut part_headers: Vec<Option<Vec<String>>> = Vec::new();
    for path in &paths {
        // Polars reads no byte order mark, and orders the keys of uneven rows by its own rule
        let is_native = is_default && !get_has_byte_order_mark(path)?;
        if is_lines
            && is_native
            && !get_is_first_item_array(path)?
            && let Some(keys) = read_first_line_keys(path)?
        {
            let table = LazyJsonLineReader::new(PlRefPath::new(path))
                .with_infer_schema_length(None)
                .finish()
                .into_result()?;
            if let Some(table) = order_first_seen_columns(table, &keys)? {
                tables.push(table);
                part_headers.push(None);
                continue;
            }
        }
        let buffer = load_file(path, None)?;
        if !is_lines
            && is_native
            && let Some(frame) = read_object_array(&buffer)
            && let Some(keys) = read_first_object_keys(&buffer)
            && let Some(table) = order_first_seen_columns(frame.lazy(), &keys)?
        {
            tables.push(table);
            part_headers.push(None);
            continue;
        }
        let document = decode_json_buffer(&buffer, is_lines)?;
        let Some(items) = get_items_from_json(document, &file_dialect)? else {
            return Ok(None);
        };

        // Array rows compare their header rows across parts before any row is read;
        // object rows have none
        let header = match &items {
            JsonItems::Rows(rows) if paths.len() > 1 => {
                get_header_labels_from_rows(rows, Some(&file_dialect))?
            }
            _ => None,
        };
        let is_compared = header.is_some();
        part_headers.push(header);
        let headers: Vec<Vec<String>> = part_headers.iter().flatten().cloned().collect();
        if is_compared && headers.len() == part_headers.len() {
            assert_part_headers(&headers)?;
        }

        let records = match items {
            JsonItems::Rows(rows) => get_records_from_rows(&rows, Some(&file_dialect))?,
            JsonItems::Records(records) => records,
        };
        tables.push(create_table_from_records(records, &file_dialect)?);
    }

    let mut result = match tables.len() {
        1 => tables.into_iter().next().unwrap_or_default(),
        _ => concat_diagonal_tables(tables)?,
    };

    if options.denormalized != Some(true) {
        let table_schema = match resolve_table_schema(resource.table_schema.as_ref())? {
            Some(table_schema) => table_schema,
            None => {
                infer_table_schema_from_table(result.clone(), &options.infer_table_schema_options)?
            }
        };
        result = normalize_table(result, &table_schema)?;
    }

    Ok(Some(result))
}

/// Whether the dialect sets nothing beyond its format, title, description and object rows.
fn get_is_default_dialect(file_dialect: &FileDialect) -> bool {
    let Ok(Value::Object(descriptor)) = serde_json::to_value(file_dialect) else {
        return true;
    };
    descriptor.iter().all(|(key, value)| match key.as_str() {
        "format" | "title" | "description" | "$schema" => true,
        "rowType" => value == "object",
        _ => false,
    })
}

/// A default dialect still admits a top-level object (not a table) or array rows,
/// so only a document opening with `[{` is read natively, and a refusal falls back.
fn read_object_array(buffer: &[u8]) -> Option<DataFrame> {
    let mut bytes = buffer
        .iter()
        .filter(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r'));
    if bytes.next() != Some(&b'[') || bytes.next() != Some(&b'{') {
        return None;
    }
    JsonReader::new(Cursor::new(buffer))
        .infer_schema_len(None)
        .finish()
        .ok()
}

/// Polars reads only object lines, while a default dialect takes array rows when
/// the first item is an array.
fn get_is_first_item_array(path: &str) -> Result<bool> {
    let mut stream = load_file_stream(path, None, None)?;
    let mut chunk = vec![0; 65536];
    loop {
        let size = stream
            .read(&mut chunk)
            .map_err(|error| Error::Load(error.to_string()))?;
        if size == 0 {
            return Ok(false);
        }
        if let Some(byte) = chunk[..size]
            .iter()
            .find(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r'))
        {
            return Ok(*byte == b'[');
        }
    }
}

fn create_table_from_records(
    records: Vec<DataRecord>,
    file_dialect: &FileDialect,
) -> Result<Table> {
    let column_names = match file_dialect {
        FileDialect::Json(dialect) => dialect.column_names.as_ref(),
        FileDialect::Jsonl(dialect) => dialect.column_names.as_ref(),
        _ => None,
    };
    // An empty array has no records to take the columns from
    if let Some(names) = column_names.filter(|names| records.is_empty() && !names.is_empty()) {
        let schema: Schema = names
            .iter()
            .map(|name| Field::new(name.as_str().into(), DataType::String))
            .collect();
        return Ok(DataFrame::empty_with_schema(&schema).lazy());
    }
    let mut keys: Vec<String> = Vec::new();
    for record in &records {
        for key in record.keys() {
            if !keys.contains(key) {
                keys.push(key.clone());
            }
        }
    }
    let text = serde_json::to_vec(&records).map_err(|error| Error::Parse(error.to_string()))?;
    let table = JsonReader::new(Cursor::new(text))
        .infer_schema_len(None)
        .finish()
        .into_result()?
        .lazy();
    Ok(order_first_seen_columns(table.clone(), &keys)?.unwrap_or(table))
}

/// The union of the keys is the first row's keys in their order unless a later row
/// adds one, whose place then needs every row read in order.
fn order_first_seen_columns(mut table: Table, keys: &[String]) -> Result<Option<Table>> {
    let schema = table.collect_schema().into_result()?;
    let is_same =
        schema.len() == keys.len() && keys.iter().all(|key| schema.contains(key.as_str()));
    if !is_same {
        return Ok(None);
    }
    let exprs: Vec<Expr> = keys.iter().map(|key| col(key.as_str())).collect();
    Ok(Some(table.select(exprs)))
}

fn get_has_byte_order_mark(path: &str) -> Result<bool> {
    let mut stream = load_file_stream(path, None, None)?;
    let mut start = [0; 3];
    let mut size = 0;
    while size < 3 {
        let read = stream
            .read(&mut start[size..])
            .map_err(|error| Error::Load(error.to_string()))?;
        if read == 0 {
            break;
        }
        size += read;
    }
    Ok(size == 3 && start == *b"\xEF\xBB\xBF")
}

fn read_first_line_keys(path: &str) -> Result<Option<Vec<String>>> {
    let mut stream = load_file_stream(path, None, None)?;
    let mut line: Vec<u8> = Vec::new();
    let mut chunk = vec![0; 65536];
    loop {
        let size = stream
            .read(&mut chunk)
            .map_err(|error| Error::Load(error.to_string()))?;
        if size == 0 {
            break;
        }
        if let Some(end) = chunk[..size].iter().position(|byte| *byte == b'\n') {
            line.extend_from_slice(&chunk[..end]);
            break;
        }
        line.extend_from_slice(&chunk[..size]);
    }
    Ok(get_object_keys(&line))
}

fn read_first_object_keys(data: &[u8]) -> Option<Vec<String>> {
    let start = data.iter().position(|byte| *byte == b'{')?;
    let mut depth = 0;
    let mut is_string = false;
    let mut index = start;
    while index < data.len() {
        let byte = data[index];
        if is_string {
            if byte == b'\\' {
                index += 1;
            } else if byte == b'"' {
                is_string = false;
            }
        } else if byte == b'"' {
            is_string = true;
        } else if byte == b'{' || byte == b'[' {
            depth += 1;
        } else if byte == b'}' || byte == b']' {
            depth -= 1;
            if depth == 0 {
                return get_object_keys(&data[start..=index]);
            }
        }
        index += 1;
    }
    None
}

fn get_object_keys(text: &[u8]) -> Option<Vec<String>> {
    match serde_json::from_slice::<Value>(text) {
        Ok(Value::Object(object)) => Some(object.keys().cloned().collect()),
        _ => None,
    }
}

/// The items of a JSON document: array rows still to be read through the header, or
/// object rows.
enum JsonItems {
    Rows(Vec<DataRow>),
    Records(Vec<DataRecord>),
}

fn get_items_from_json(document: Value, file_dialect: &FileDialect) -> Result<Option<JsonItems>> {
    let (json_pointer, row_type, column_names) = match file_dialect {
        FileDialect::Json(dialect) => (
            dialect.json_pointer.clone(),
            dialect.row_type,
            dialect.column_names.clone(),
        ),
        FileDialect::Jsonl(dialect) => (None, dialect.row_type, dialect.column_names.clone()),
        _ => return Ok(None),
    };

    let data = match json_pointer {
        Some(pointer) => get_json_pointer_value(document, &pointer)?,
        None => Some(document),
    };
    let Some(Value::Array(data)) = data else {
        return Ok(None);
    };

    let row_type = row_type.unwrap_or(match data.first() {
        Some(Value::Array(_)) => RowType::Array,
        _ => RowType::Object,
    });

    if row_type == RowType::Array {
        let rows: Vec<DataRow> = data
            .into_iter()
            .filter_map(|row| match row {
                Value::Array(row) => Some(row),
                _ => None,
            })
            .collect();
        return Ok(Some(JsonItems::Rows(rows)));
    }

    let records = data.into_iter().filter_map(|row| match row {
        Value::Object(record) => Some(record),
        _ => None,
    });
    let records = match column_names.filter(|names| !names.is_empty()) {
        None => records.collect(),
        Some(names) => records
            .map(|record| {
                names
                    .iter()
                    .map(|name| {
                        (
                            name.clone(),
                            record.get(name).cloned().unwrap_or(Value::Null),
                        )
                    })
                    .collect()
            })
            .collect(),
    };
    Ok(Some(JsonItems::Records(records)))
}

fn get_json_pointer_value(document: Value, pointer: &str) -> Result<Option<Value>> {
    let mut value = document;
    for token in parse_json_pointer(pointer)? {
        value = match value {
            Value::Array(mut items) => {
                let index = token
                    .parse::<usize>()
                    .ok()
                    .filter(|_| token.chars().all(|char| char.is_ascii_digit()));
                match index {
                    Some(index) if index < items.len() => items.swap_remove(index),
                    _ => return Ok(None),
                }
            }
            Value::Object(mut object) => match object.remove(&token) {
                Some(value) => value,
                None => return Ok(None),
            },
            _ => return Ok(None),
        };
    }
    Ok(Some(value))
}
