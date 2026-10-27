use serde_json::Value;

/// Normalize a Frictionless field list, which may be a single name
pub fn convert_field_names(names: &Value) -> Value {
    match names {
        Value::String(_) => Value::Array(vec![names.clone()]),
        _ => names.clone(),
    }
}
