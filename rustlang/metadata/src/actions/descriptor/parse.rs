use serde_json::Value;

use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};

pub fn parse_descriptor(text: &str) -> Result<Descriptor> {
    let value: Value =
        serde_json::from_str(text).map_err(|error| Error::Parse(error.to_string()))?;
    match value {
        Value::Object(descriptor) => Ok(descriptor),
        _ => Err(Error::Parse(format!("Invalid descriptor: {text}"))),
    }
}
