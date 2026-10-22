use fairspec_metadata::{JsonSchema, Resource, Result};
use serde_json::{Map, Value, json};

use crate::actions::data::load::load_data;

pub fn infer_data_schema(resource: &Resource) -> Result<Option<JsonSchema>> {
    let Some(data) = load_data(resource)? else {
        return Ok(None);
    };

    match infer_value_schema(&data) {
        Value::Object(schema) => Ok(Some(schema)),
        _ => Ok(None),
    }
}

fn infer_value_schema(value: &Value) -> Value {
    match value {
        Value::Null => json!({"type": "null"}),
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Number(number) if number.is_f64() => {
            let is_integer = number.as_f64().is_some_and(|value| value.fract() == 0.0);
            json!({"type": if is_integer { "integer" } else { "number" }})
        }
        Value::Number(_) => json!({"type": "integer"}),
        Value::String(_) => json!({"type": "string"}),
        Value::Array(items) => {
            let items: Vec<Value> = items.iter().map(infer_value_schema).collect();
            match items.first() {
                Some(first) if items.iter().all(|item| item == first) => {
                    json!({"type": "array", "items": first})
                }
                _ => json!({"type": "array"}),
            }
        }
        Value::Object(object) if object.is_empty() => json!({"type": "object"}),
        Value::Object(object) => {
            let properties: Map<String, Value> = object
                .iter()
                .map(|(key, item)| (key.clone(), infer_value_schema(item)))
                .collect();
            json!({"type": "object", "properties": properties})
        }
    }
}
