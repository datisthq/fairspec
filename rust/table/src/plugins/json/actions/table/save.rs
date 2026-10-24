use fairspec_dataset::{assert_local_path_vacant, save_file};
use fairspec_metadata::{Error, FileDialect, PathOr, Resource, Result, get_supported_file_dialect};
use polars::prelude::*;
use serde_json::{Map, Value};

use crate::actions::table::denormalize::denormalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::table::LazyFrameExt;
use crate::models::column::DenormalizeColumnOptions;
use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::{SaveTableOptions, Table};
use crate::plugins::json::actions::buffer::decode::decode_json_buffer;
use crate::plugins::json::actions::buffer::encode::encode_json_buffer;
use crate::plugins::json::helpers::pointer::parse_json_pointer;
use crate::plugins::json::settings::NATIVE_TYPES;

pub fn save_json_table(table: Table, options: &SaveTableOptions) -> Result<String> {
    let path = &options.path;
    let overwrite = options.overwrite == Some(true);
    if !overwrite {
        assert_local_path_vacant(path)?;
    }

    let resource = Resource {
        data: Some(Value::String(path.clone())),
        file_dialect: options.file_dialect.clone().map(PathOr::Value),
        ..Resource::default()
    };
    let Some(file_dialect) = get_supported_file_dialect(&resource, &["json", "jsonl"])? else {
        return Err(Error::Load("Saving options is not compatible".to_string()));
    };
    let is_lines = file_dialect.format() == Some("jsonl");

    let table_schema = match &options.table_schema {
        Some(table_schema) => table_schema.clone(),
        None => infer_table_schema_from_table(
            table.clone(),
            &InferTableSchemaOptions {
                table_schema_options: options.table_schema_options.clone(),
                keep_strings: Some(true),
                ..InferTableSchemaOptions::default()
            },
        )?,
    };
    let denormalize_options = DenormalizeColumnOptions {
        native_types: Some(NATIVE_TYPES.iter().map(|value| value.to_string()).collect()),
    };
    let table = denormalize_table(table, &table_schema, &denormalize_options)?;

    let mut frame = table.collect_table()?;
    let mut text = Vec::new();
    let format = if is_lines {
        JsonFormat::JsonLines
    } else {
        JsonFormat::Json
    };
    JsonWriter::new(&mut text)
        .with_json_format(format)
        .finish(&mut frame)
        .into_result()?;

    let data = decode_json_buffer(&text, is_lines)?;
    let data = process_data(data, &file_dialect)?;
    save_file(path, &encode_json_buffer(&data, is_lines), overwrite)?;

    Ok(path.clone())
}

fn process_data(records: Value, file_dialect: &FileDialect) -> Result<Value> {
    let (json_pointer, row_type, column_names) = match file_dialect {
        FileDialect::Json(dialect) => (
            dialect.json_pointer.clone(),
            dialect.row_type,
            dialect.column_names.clone(),
        ),
        FileDialect::Jsonl(dialect) => (None, dialect.row_type, dialect.column_names.clone()),
        _ => (None, None, None),
    };
    let column_names = column_names.filter(|names| !names.is_empty());
    let mut data = records;

    if let Some(names) = &column_names {
        data = Value::Array(
            get_rows(&data)
                .iter()
                .map(|row| {
                    names
                        .iter()
                        .map(|name| Ok((name.clone(), get_field(row, name)?)))
                        .collect::<Result<Map<_, _>>>()
                        .map(Value::Object)
                })
                .collect::<Result<_>>()?,
        );
    }

    if row_type == Some(fairspec_metadata::RowType::Array) {
        let rows = get_rows(&data);
        let names: Vec<String> = match &column_names {
            Some(names) => names.clone(),
            None => rows
                .first()
                .and_then(Value::as_object)
                .ok_or_else(|| Error::Load("Table has no rows to save".to_string()))?
                .keys()
                .cloned()
                .collect(),
        };
        let mut array = vec![Value::Array(
            names.iter().cloned().map(Value::String).collect(),
        )];
        for row in rows {
            let values = names
                .iter()
                .map(|name| get_field(row, name))
                .collect::<Result<_>>()?;
            array.push(Value::Array(values));
        }
        data = Value::Array(array);
    }

    if let Some(pointer) = json_pointer.filter(|pointer| !pointer.is_empty()) {
        for token in parse_json_pointer(&pointer)?.into_iter().rev() {
            data = Value::Object(Map::from_iter([(token, data)]));
        }
    }

    Ok(data)
}

fn get_rows(data: &Value) -> &[Value] {
    data.as_array().map(Vec::as_slice).unwrap_or_default()
}

fn get_field(row: &Value, name: &str) -> Result<Value> {
    row.get(name)
        .cloned()
        .ok_or_else(|| Error::Load(format!("Column \"{name}\" is not in the table")))
}
