use serde_json::{Map, Value};

pub type DataRow = Vec<Value>;
pub type DataRecord = Map<String, Value>;
