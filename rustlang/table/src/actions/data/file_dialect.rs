use fairspec_metadata::FileDialect;
use serde_json::{Map, Value};

use crate::helpers::file_dialect::{get_header_rows, join_header_labels};
use crate::helpers::general::format_python_float;
use crate::models::data::{DataRecord, DataRow};

pub fn get_records_from_rows(
    rows: &[DataRow],
    file_dialect: Option<&FileDialect>,
) -> Vec<DataRecord> {
    let options = get_dialect_options(file_dialect);
    let header = get_header_from_rows(rows, file_dialect, &options);
    let content = get_content_from_rows(rows, file_dialect, &options);

    let Some(labels) = get_labels_from_header(&header, &options) else {
        return vec![];
    };

    content
        .into_iter()
        .filter(|row| !get_is_commented_row(row, &options))
        .map(|row| {
            labels
                .iter()
                .zip(row)
                .map(|(label, value)| (label.clone(), value.clone()))
                .collect::<Map<String, Value>>()
        })
        .collect()
}

/// The fields every dialect with header and comment rows shares, read off its descriptor.
struct DialectOptions {
    column_names: Option<Vec<String>>,
    header_join: Option<String>,
    comment_rows: Vec<usize>,
    comment_prefix: Option<String>,
}

fn get_dialect_options(file_dialect: Option<&FileDialect>) -> DialectOptions {
    let descriptor = file_dialect
        .and_then(|dialect| serde_json::to_value(dialect).ok())
        .unwrap_or_default();
    let get = |key: &str| descriptor.get(key).cloned().unwrap_or_default();
    DialectOptions {
        column_names: serde_json::from_value(get("columnNames")).ok(),
        header_join: serde_json::from_value(get("headerJoin")).ok(),
        comment_rows: serde_json::from_value(get("commentRows")).unwrap_or_default(),
        comment_prefix: serde_json::from_value(get("commentPrefix")).ok(),
    }
}

fn get_header_from_rows(
    rows: &[DataRow],
    file_dialect: Option<&FileDialect>,
    options: &DialectOptions,
) -> Vec<DataRow> {
    if let Some(column_names) = &options.column_names {
        return vec![column_names.iter().cloned().map(Value::String).collect()];
    }

    let header_rows = get_header_rows(file_dialect);
    if header_rows.is_empty() {
        let length = rows.iter().map(Vec::len).max().unwrap_or(0);
        return vec![
            (1..=length)
                .map(|index| Value::String(format!("column{index}")))
                .collect(),
        ];
    }

    header_rows
        .iter()
        .filter_map(|number| rows.get(number.checked_sub(1)?).cloned())
        .collect()
}

fn get_content_from_rows<'a>(
    rows: &'a [DataRow],
    file_dialect: Option<&FileDialect>,
    options: &DialectOptions,
) -> Vec<&'a DataRow> {
    let header_rows = get_header_rows(file_dialect);
    let skip_rows = header_rows.first().map_or(0, |row| row.saturating_sub(1));

    rows.iter()
        .enumerate()
        .filter(|(index, row)| {
            let number = index + 1;
            number > skip_rows
                && !header_rows.contains(&number)
                && !options.comment_rows.contains(&number)
                && !get_is_commented_row(row, options)
        })
        .map(|(_, row)| row)
        .collect()
}

fn get_labels_from_header(header: &[DataRow], options: &DialectOptions) -> Option<Vec<String>> {
    match header {
        [] => None,
        [labels] => Some(labels.iter().map(format_python_str).collect()),
        _ => Some(join_header_labels(
            header,
            options.header_join.as_deref().unwrap_or(" "),
        )),
    }
}

fn get_is_commented_row(row: &[Value], options: &DialectOptions) -> bool {
    match (&options.comment_prefix, row.first()) {
        (Some(prefix), Some(Value::String(first))) => first.starts_with(prefix.as_str()),
        _ => false,
    }
}

/// Python's `str()` of a JSON value.
fn format_python_str(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(number) if number.is_f64() => {
            format_python_float(number.as_f64().unwrap_or_default())
        }
        other => other.to_string(),
    }
}
