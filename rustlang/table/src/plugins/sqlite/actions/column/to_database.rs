use fairspec_metadata::{Column, ColumnType};
use serde_json::Value;

use crate::plugins::sqlite::models::column::SqliteColumn;

pub fn convert_column_to_database(column: &Column, is_nullable: bool) -> SqliteColumn {
    let property = serde_json::to_value(column.property()).unwrap_or_default();
    let comment = property
        .get("description")
        .and_then(Value::as_str)
        .map(String::from);

    SqliteColumn {
        is_nullable,
        comment,
        ..SqliteColumn::new(column.name(), convert_type(column.column_type()))
    }
}

fn convert_type(column_type: ColumnType) -> &'static str {
    match column_type {
        ColumnType::Boolean | ColumnType::Integer => "integer",
        ColumnType::Number => "real",
        _ => "text",
    }
}
