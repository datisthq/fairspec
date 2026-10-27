mod column_property_unit;
mod dataset_unit;
mod error_unit;
mod file_dialect_unit;
mod fixtures_unit;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

fn round_trip<T: Serialize + DeserializeOwned>(input: &Value) -> Value {
    let model: T = serde_json::from_value(input.clone())
        .unwrap_or_else(|error| panic!("failed to parse {input}: {error}"));
    serde_json::to_value(&model).unwrap()
}

fn parse<T: DeserializeOwned>(input: Value) -> T {
    serde_json::from_value(input).unwrap()
}
