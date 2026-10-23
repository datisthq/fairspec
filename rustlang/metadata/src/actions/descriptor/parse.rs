use serde_json::Value;

use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};

pub fn parse_descriptor(text: &str) -> Result<Descriptor> {
    let value: Value =
        serde_json::from_str(text).map_err(|error| Error::Parse(error.to_string()))?;
    match value {
        Value::Object(descriptor) => Ok(descriptor),
        value => Err(Error::Parse(format!(
            "Invalid descriptor: expected a JSON object, got {}",
            get_json_type(&value)
        ))),
    }
}

fn get_json_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
