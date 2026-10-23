use fairspec_dataset::assert_local_path_vacant;
use fairspec_metadata::{Error, FileDialect, PathOr, Resource, Result, get_supported_file_dialect};
use polars::prelude::*;
use serde_json::Value;

use crate::actions::table::denormalize::denormalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::file_dialect::get_header_rows;
use crate::helpers::table::LazyFrameExt;
use crate::models::column::DenormalizeColumnOptions;
use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::{SaveTableOptions, Table};
use crate::plugins::csv::settings::NATIVE_TYPES;

pub fn save_csv_table(table: Table, options: &SaveTableOptions) -> Result<String> {
    let path = &options.path;
    if options.overwrite != Some(true) {
        assert_local_path_vacant(path)?;
    }

    let resource = Resource {
        data: Some(Value::String(path.clone())),
        file_dialect: options.file_dialect.clone().map(PathOr::Value),
        ..Resource::default()
    };
    let Some(file_dialect) = get_supported_file_dialect(&resource, &["csv", "tsv"])? else {
        return Err(Error::Load("Saving options is not compatible".to_string()));
    };
    let header_rows = get_header_rows(Some(&file_dialect));

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

    let (separator, quote_char, line_terminator) = match &file_dialect {
        FileDialect::Csv(dialect) => (
            dialect
                .delimiter
                .as_deref()
                .and_then(|d| d.bytes().next())
                .unwrap_or(b','),
            dialect
                .quote_char
                .as_deref()
                .and_then(|q| q.bytes().next())
                .unwrap_or(b'"'),
            dialect.line_terminator.clone(),
        ),
        FileDialect::Tsv(dialect) => (b'\t', b'"', dialect.line_terminator.clone()),
        _ => (b',', b'"', None),
    };

    let serialize_options = SerializeOptions {
        separator,
        quote_char,
        line_terminator: line_terminator
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| "\n".to_string())
            .into(),
        ..SerializeOptions::default()
    };
    // A `.tsv` path fails the extension check that `sink_csv` skips in py-polars
    let csv_options = CsvWriterOptions {
        include_header: !header_rows.is_empty(),
        check_extension: false,
        serialize_options: Arc::new(serialize_options),
        ..CsvWriterOptions::default()
    };
    table.sink_table(path, FileWriteFormat::Csv(csv_options))?;

    Ok(path.clone())
}
