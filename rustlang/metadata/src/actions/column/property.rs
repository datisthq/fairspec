use indexmap::IndexMap;
use serde_json::Value;

use crate::models::column::column::{Column, ColumnProperty};

pub fn get_base_property_type(r#type: Option<&Value>) -> Option<&str> {
    match r#type? {
        Value::String(name) => Some(name),
        Value::Array(names) => Some(
            names
                .iter()
                .filter_map(Value::as_str)
                .find(|name| *name != "null")
                .unwrap_or("null"),
        ),
        _ => None,
    }
}

pub fn get_is_nullable_property_type(r#type: Option<&Value>) -> bool {
    match r#type {
        Some(Value::Array(names)) => names.iter().any(|name| name == "null"),
        _ => false,
    }
}

pub fn get_column_properties(columns: &[Column]) -> IndexMap<String, ColumnProperty> {
    columns
        .iter()
        .map(|column| (column.name().to_string(), column.property()))
        .collect()
}

/// Widens a column property type to include `"null"`.
pub fn set_property_nullable(column: &mut Column) {
    let mut property = column.property();
    if property.is_nullable() {
        return;
    }
    property.set_nullable();
    column.set_property(property);
}
