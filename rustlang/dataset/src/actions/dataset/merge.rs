use fairspec_metadata::{Dataset, Error, Result, load_dataset_descriptor};
use serde_json::Value;

/// The system dataset with the user dataset's top-level properties laid over it.
pub fn merge_datasets(
    system_dataset: &Dataset,
    user_dataset_path: Option<&str>,
) -> Result<Dataset> {
    let mut merged = to_object(system_dataset)?;
    if let Some(path) = user_dataset_path.filter(|path| !path.is_empty()) {
        merged.extend(to_object(&load_dataset_descriptor(path)?)?);
    }
    serde_json::from_value(Value::Object(merged)).map_err(|error| Error::Parse(error.to_string()))
}

fn to_object(dataset: &Dataset) -> Result<serde_json::Map<String, Value>> {
    match serde_json::to_value(dataset) {
        Ok(Value::Object(object)) => Ok(object),
        Ok(_) => Ok(serde_json::Map::new()),
        Err(error) => Err(Error::Parse(error.to_string())),
    }
}
