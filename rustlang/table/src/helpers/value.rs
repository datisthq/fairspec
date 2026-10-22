use polars::prelude::AnyValue;
use serde_json::{Map, Number, Value};

/// A polars cell as JSON, the shape Python's `to_dicts` and `json.dumps` give it;
/// temporal values come out as polars' display text (`2025-01-01 00:00:00`), which is
/// Python's `str()` of the value at whole seconds — the form header labels need.
pub fn get_json_value(value: &AnyValue) -> Value {
    match value {
        AnyValue::Null => Value::Null,
        AnyValue::Boolean(value) => Value::Bool(*value),
        AnyValue::String(value) => Value::String(value.to_string()),
        AnyValue::StringOwned(value) => Value::String(value.to_string()),
        AnyValue::Int8(value) => Value::from(*value),
        AnyValue::Int16(value) => Value::from(*value),
        AnyValue::Int32(value) => Value::from(*value),
        AnyValue::Int64(value) => Value::from(*value),
        AnyValue::UInt8(value) => Value::from(*value),
        AnyValue::UInt16(value) => Value::from(*value),
        AnyValue::UInt32(value) => Value::from(*value),
        AnyValue::UInt64(value) => Value::from(*value),
        AnyValue::Float32(value) => get_float_value(f64::from(*value)),
        AnyValue::Float64(value) => get_float_value(*value),
        AnyValue::List(series) => {
            Value::Array(series.iter().map(|item| get_json_value(&item)).collect())
        }
        AnyValue::StructOwned(payload) => {
            let (values, fields) = payload.as_ref();
            let object: Map<String, Value> = fields
                .iter()
                .zip(values)
                .map(|(field, value)| (field.name().to_string(), get_json_value(value)))
                .collect();
            Value::Object(object)
        }
        other => match other.get_str() {
            Some(text) => Value::String(text.to_string()),
            None => Value::String(other.to_string()),
        },
    }
}

fn get_float_value(value: f64) -> Value {
    Number::from_f64(value).map_or(Value::Null, Value::Number)
}
