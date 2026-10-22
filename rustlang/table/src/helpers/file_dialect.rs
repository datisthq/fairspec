use fairspec_metadata::{FileDialect, HeaderRows};
use serde_json::Value;

use crate::models::data::DataRow;
use crate::models::file_dialect::FileDialectWithHeaderAndCommentRows;

pub fn get_header_rows(file_dialect: Option<&FileDialectWithHeaderAndCommentRows>) -> Vec<usize> {
    let header_rows = match file_dialect {
        Some(FileDialect::Csv(dialect)) => dialect.header_rows.as_ref(),
        Some(FileDialect::Tsv(dialect)) => dialect.header_rows.as_ref(),
        Some(FileDialect::Json(dialect)) => dialect.header_rows.as_ref(),
        Some(FileDialect::Jsonl(dialect)) => dialect.header_rows.as_ref(),
        Some(FileDialect::Xlsx(dialect)) => dialect.header_rows.as_ref(),
        Some(FileDialect::Ods(dialect)) => dialect.header_rows.as_ref(),
        _ => None,
    };
    match header_rows {
        None => vec![1],
        Some(HeaderRows::Disabled) => Vec::new(),
        Some(HeaderRows::Rows(rows)) => rows.clone(),
    }
}

pub fn get_comment_rows(file_dialect: &FileDialectWithHeaderAndCommentRows) -> Vec<usize> {
    let comment_rows = match file_dialect {
        FileDialect::Csv(dialect) => dialect.comment_rows.as_ref(),
        FileDialect::Tsv(dialect) => dialect.comment_rows.as_ref(),
        FileDialect::Json(dialect) => dialect.comment_rows.as_ref(),
        FileDialect::Jsonl(dialect) => dialect.comment_rows.as_ref(),
        FileDialect::Xlsx(dialect) => dialect.comment_rows.as_ref(),
        FileDialect::Ods(dialect) => dialect.comment_rows.as_ref(),
        _ => None,
    };
    comment_rows.cloned().unwrap_or_default()
}

/// A label carries forward across the empty cells to its right, so a group label
/// spanning several columns prefixes each of them.
pub fn join_header_labels(header: &[DataRow], header_join: &str) -> Vec<String> {
    let width = header.iter().map(Vec::len).max().unwrap_or(0);
    let rows: Vec<Vec<String>> = header
        .iter()
        .map(|row| fill_header_row(row, width))
        .collect();
    (0..width)
        .map(|index| {
            rows.iter()
                .map(|row| row[index].as_str())
                .filter(|label| !label.is_empty())
                .collect::<Vec<_>>()
                .join(header_join)
        })
        .collect()
}

fn fill_header_row(row: &DataRow, width: usize) -> Vec<String> {
    let mut labels = Vec::with_capacity(width);
    let mut label = String::new();
    for index in 0..width {
        if let Some(cell) = row.get(index).filter(|cell| !cell.is_null()) {
            let text = stringify_cell(cell);
            if !text.is_empty() {
                label = text;
            }
        }
        labels.push(label.clone());
    }
    labels
}

// Python's `str()` of a cell
fn stringify_cell(cell: &Value) -> String {
    match cell {
        Value::String(text) => text.clone(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(number) if number.is_f64() => {
            super::general::format_python_float(number.as_f64().unwrap_or_default())
        }
        other => other.to_string(),
    }
}
