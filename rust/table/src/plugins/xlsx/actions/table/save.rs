use fairspec_dataset::{assert_local_path_vacant, save_file};
use fairspec_metadata::{Error, FileDialect, PathOr, Resource, Result, get_supported_file_dialect};
use polars::prelude::*;
use serde_json::Value;

use crate::actions::table::denormalize::denormalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::table::LazyFrameExt;
use crate::models::column::DenormalizeColumnOptions;
use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::{SaveTableOptions, Table};
use crate::plugins::xlsx::helpers::write::{
    SheetData, SheetValue, write_ods_buffer, write_xlsx_buffer,
};
use crate::plugins::xlsx::settings::NATIVE_TYPES;

pub fn save_xlsx_table(table: Table, options: &SaveTableOptions) -> Result<String> {
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
    let Some(file_dialect) = get_supported_file_dialect(&resource, &["xlsx", "ods"])? else {
        return Err(Error::Load("Saving options is not compatible".to_string()));
    };

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
    let frame = denormalize_table(table, &table_schema, &denormalize_options)?.collect_table()?;

    let (sheet_name, is_ods) = match &file_dialect {
        FileDialect::Xlsx(dialect) => (dialect.sheet_name.clone(), false),
        FileDialect::Ods(dialect) => (dialect.sheet_name.clone(), true),
        _ => (None, false),
    };

    let header = frame
        .get_column_names()
        .iter()
        .map(|name| Some(SheetValue::String(name.to_string())))
        .collect();
    let mut rows = vec![header];
    for index in 0..frame.height() {
        let row = frame
            .columns()
            .iter()
            .map(|column| column.get(index).map(convert_sheet_value))
            .collect::<PolarsResult<Vec<_>>>()
            .into_result()?;
        rows.push(row);
    }

    let sheets = [SheetData {
        name: sheet_name
            .filter(|name| !name.is_empty())
            .unwrap_or_else(|| "Sheet1".to_string()),
        rows,
    }];
    let buffer = if is_ods {
        write_ods_buffer(&sheets)?
    } else {
        write_xlsx_buffer(&sheets)?
    };
    save_file(path, &buffer, overwrite)?;

    Ok(path.clone())
}

fn convert_sheet_value(value: AnyValue) -> Option<SheetValue> {
    match value {
        AnyValue::Null => None,
        AnyValue::Boolean(value) => Some(SheetValue::Boolean(value)),
        AnyValue::String(value) => Some(SheetValue::String(value.to_string())),
        AnyValue::StringOwned(value) => Some(SheetValue::String(value.to_string())),
        value if value.is_primitive_numeric() => value.extract::<f64>().map(SheetValue::Number),
        value => Some(SheetValue::String(value.to_string())),
    }
}
