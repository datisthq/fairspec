use fairspec_dataset::load_file;
use fairspec_metadata::{
    Error, Resource, Result, get_data_first_path, get_data_value, get_supported_file_dialect,
};
use serde_json::Value;

pub fn load_data(resource: &Resource) -> Result<Option<Value>> {
    if let Some(data_value) = get_data_value(resource) {
        return Ok(Some(data_value.clone()));
    }

    if let Some(first_path) = get_data_first_path(resource)
        && get_supported_file_dialect(resource, &["json"])?.is_some()
    {
        let bytes = load_file(&first_path, None)?;
        let data =
            serde_json::from_slice(&bytes).map_err(|error| Error::Parse(error.to_string()))?;
        return Ok(Some(data));
    }

    Ok(None)
}
