use fairspec_metadata::Column;
use serde_json::Value;

/// The column's missing values with each `{value, label}` item flattened to its value.
pub fn get_missing_values(column: &Column) -> Vec<Value> {
    let property = serde_json::to_value(column.property()).unwrap_or_default();
    let Some(Value::Array(items)) = property.get("missingValues") else {
        return Vec::new();
    };
    items
        .iter()
        .map(|item| match item {
            Value::Object(object) => object.get("value").cloned().unwrap_or(Value::Null),
            other => other.clone(),
        })
        .collect()
}

/// A series of JSON scalars: integers when they all are, else text.
pub fn create_series(values: &[Value]) -> polars::prelude::Series {
    use polars::prelude::*;
    if values.iter().all(|value| value.as_i64().is_some()) {
        let integers: Vec<i64> = values.iter().filter_map(Value::as_i64).collect();
        return Series::new("".into(), integers);
    }
    let texts: Vec<String> = values
        .iter()
        .map(|value| match value {
            Value::String(text) => text.clone(),
            other => other.to_string(),
        })
        .collect();
    Series::new("".into(), texts)
}
