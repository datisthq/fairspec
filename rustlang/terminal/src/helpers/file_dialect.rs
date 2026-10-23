use fairspec_library::fairspec_metadata::{
    FileDialect, HeaderRows, PathOr, Resource, infer_file_dialect_format,
};
use serde_json::{Map, Value, json};

use crate::params::file_dialect::DialectParams;

/// The `--dialect` path, or a dialect built from the dialect options for the path's format.
pub fn get_file_dialect(path: &str, params: &DialectParams) -> Option<PathOr<FileDialect>> {
    if let Some(dialect) = params
        .dialect
        .as_ref()
        .filter(|dialect| !dialect.is_empty())
    {
        return Some(PathOr::Path(dialect.clone()));
    }
    create_file_dialect_from_path_and_options(path, params).map(PathOr::Value)
}

pub fn create_file_dialect_from_path_and_options(
    path: &str,
    params: &DialectParams,
) -> Option<FileDialect> {
    let resolved_format = match params.format.as_ref().filter(|format| !format.is_empty()) {
        Some(format) => Some(format.clone()),
        None => infer_file_dialect_format(&Resource {
            data: Some(Value::String(path.to_string())),
            ..Resource::default()
        })
        .map(str::to_string),
    }?;

    let mut dialect = Map::new();
    dialect.insert("format".to_string(), json!(resolved_format));
    match resolved_format.as_str() {
        "csv" | "tsv" => {
            insert_text(&mut dialect, "lineTerminator", &params.line_terminator);
            insert_text(&mut dialect, "nullSequence", &params.null_sequence);
            insert_header_options(&mut dialect, params);
            if let Some(names) = params
                .column_names
                .as_ref()
                .filter(|names| !names.0.is_empty())
            {
                dialect.insert("columnNames".to_string(), json!(names.0));
            }
            if resolved_format == "csv" {
                insert_text(&mut dialect, "delimiter", &params.delimiter);
                insert_text(&mut dialect, "quoteChar", &params.quote_char);
            }
        }
        "xlsx" | "ods" => {
            if let Some(sheet_number) = params.sheet_number {
                dialect.insert("sheetNumber".to_string(), json!(sheet_number));
            }
            insert_text(&mut dialect, "sheetName", &params.sheet_name);
            insert_header_options(&mut dialect, params);
        }
        "json" | "jsonl" => {
            insert_header_options(&mut dialect, params);
            insert_text(&mut dialect, "rowType", &params.row_type);
            if resolved_format == "json" {
                insert_text(&mut dialect, "jsonPointer", &params.json_pointer);
            }
        }
        "sqlite" => insert_text(&mut dialect, "tableName", &params.table_name),
        _ => return None,
    }

    serde_json::from_value(Value::Object(dialect)).ok()
}

// headerRows, headerJoin, commentRows and commentPrefix, in Python's key order
fn insert_header_options(dialect: &mut Map<String, Value>, params: &DialectParams) {
    match &params.header_rows {
        Some(HeaderRows::Disabled) => {
            dialect.insert("headerRows".to_string(), json!(false));
        }
        Some(HeaderRows::Rows(rows)) => {
            dialect.insert("headerRows".to_string(), json!(rows));
        }
        None => {}
    }
    insert_text(dialect, "headerJoin", &params.header_join);
    if let Some(rows) = params
        .comment_rows
        .as_ref()
        .filter(|rows| !rows.0.is_empty())
    {
        dialect.insert("commentRows".to_string(), json!(rows.0));
    }
    insert_text(dialect, "commentPrefix", &params.comment_prefix);
}

fn insert_text(dialect: &mut Map<String, Value>, key: &str, value: &Option<String>) {
    if let Some(value) = value.as_ref().filter(|value| !value.is_empty()) {
        dialect.insert(key.to_string(), json!(value));
    }
}
