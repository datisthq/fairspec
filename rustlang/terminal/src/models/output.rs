use std::fmt;

use serde::Serialize;
use serde_json::{Map, Value};

#[fireargs::output]
pub struct ReportResult {
    /// Whether the validation passed without errors
    pub valid: bool,
    /// Array of validation errors encountered
    pub errors: Vec<Value>,
}

#[fireargs::output]
pub struct TextResult {
    pub result: String,
}

#[fireargs::output]
pub struct Empty {}

pub type Records = Vec<Value>;

pub type Descriptor = Map<String, Value>;

/// A command that failed after writing its human output; under `--json`, `data` is
/// the result to print before exiting with code 1.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Exit {
    pub data: Option<Value>,
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command failed")
    }
}

impl std::error::Error for Exit {}

/// A typed library result as the object `--json` prints.
pub fn create_descriptor(value: &impl Serialize) -> Descriptor {
    match serde_json::to_value(value) {
        Ok(Value::Object(object)) => object,
        _ => Descriptor::new(),
    }
}
