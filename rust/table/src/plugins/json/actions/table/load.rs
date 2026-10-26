use std::io::Cursor;

use fairspec_dataset::{load_file, prefetch_files};
use fairspec_metadata::{
    Error, FileDialect, PathOr, Resource, Result, RowType, get_supported_file_dialect,
    resolve_table_schema,
};
use polars::prelude::*;
use serde_json::Value;

use crate::actions::data::file_dialect::get_records_from_rows;
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
    for path in &paths {
        if is_lines && is_default {
            tables.push(
                LazyJsonLineReader::new(PlRefPath::new(path))
                    .finish()
                    .into_result()?,
            );
            continue;
        }
        let document = decode_json_buffer(&load_file(path, None)?, is_lines)?;
        let Some(records) = get_records_from_json(document, &file_dialect)? else {
            return Ok(None);
        };
        tables.push(create_table_from_records(records)?);
    }

    let mut result = match tables.len() {
        1 => tables.into_iter().next().unwrap_or_default(),
        _ => concat(tables, UnionArgs::default()).into_result()?,
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

/// Whether the dialect sets nothing beyond its format, title and description.
fn get_is_default_dialect(file_dialect: &FileDialect) -> bool {
    let Ok(Value::Object(descriptor)) = serde_json::to_value(file_dialect) else {
        return true;
    };
    descriptor
        .keys()
        .all(|key| matches!(key.as_str(), "format" | "title" | "description" | "$schema"))
}

fn create_table_from_records(records: Vec<DataRecord>) -> Result<Table> {
    let text = serde_json::to_vec(&records).map_err(|error| Error::Parse(error.to_string()))?;
    Ok(JsonReader::new(Cursor::new(text))
        .infer_schema_len(None)
        .finish()
        .into_result()?
        .lazy())
}

fn get_records_from_json(
    document: Value,
    file_dialect: &FileDialect,
) -> Result<Option<Vec<DataRecord>>> {
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
        return Ok(Some(get_records_from_rows(&rows, Some(file_dialect))));
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
    Ok(Some(records))
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
