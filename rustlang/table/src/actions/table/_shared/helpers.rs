use fairspec_metadata::{Column, TableSchema, get_base_property_type};
use serde_json::Value;

/// The column with the table-wide missing values appended; a string column takes only text
/// ones. An item the column's property cannot hold (a float for an integer) is left out,
/// where Python appends it unchecked: substitution could never match it anyway.
pub fn merge_missing_values(column: &Column, table_schema: &TableSchema) -> Column {
    let Some(table_missing_values) = table_schema
        .missing_values
        .as_ref()
        .filter(|v| !v.is_empty())
    else {
        return column.clone();
    };

    let mut merged = column.clone();
    for item in table_missing_values {
        let item = serde_json::to_value(item).unwrap_or_default();
        let property = serde_json::to_value(merged.property()).unwrap_or_default();
        if get_base_property_type(property.get("type")) == Some("string") {
            let value = item.get("value").unwrap_or(&item);
            if !value.is_string() {
                continue;
            }
        }
        if let Some(candidate) = append_missing_value(&merged, item) {
            merged = candidate;
        }
    }
    merged
}

fn append_missing_value(column: &Column, item: Value) -> Option<Column> {
    let mut descriptor = serde_json::to_value(column).ok()?;
    let property = descriptor.get_mut("property")?.as_object_mut()?;
    let missing_values = property
        .entry("missingValues")
        .or_insert_with(|| Value::Array(Vec::new()));
    missing_values.as_array_mut()?.push(item);
    serde_json::from_value(descriptor).ok()
}
