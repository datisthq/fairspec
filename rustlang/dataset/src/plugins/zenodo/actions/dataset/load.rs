use fairspec_metadata::{Descriptor, Error, Result};
use serde_json::Value;

use super::from_zenodo::convert_dataset_from_zenodo;
use crate::actions::dataset::merge::merge_datasets;
use crate::plugins::zenodo::models::record::ZenodoRecord;
use crate::plugins::zenodo::services::zenodo::make_zenodo_api_request;

pub fn load_dataset_from_zenodo(dataset_url: &str, api_key: Option<&str>) -> Result<Descriptor> {
    let parsed = url::Url::parse(dataset_url).ok();
    let sandbox = parsed.as_ref().and_then(url::Url::host_str) == Some("sandbox.zenodo.org");

    let record_id = parsed
        .as_ref()
        .and_then(|url| {
            url.path()
                .split('/')
                .rfind(|part| !part.is_empty())
                .map(String::from)
        })
        .ok_or_else(|| {
            Error::Load(format!(
                "Failed to extract record ID from URL: {dataset_url}"
            ))
        })?;

    let zenodo_record: ZenodoRecord = make_zenodo_api_request(
        &format!("/records/{record_id}"),
        None,
        None,
        None,
        api_key,
        sandbox,
    )?;

    let system_dataset = convert_dataset_from_zenodo(&zenodo_record);
    let user_dataset_path = system_dataset
        .resources
        .iter()
        .flatten()
        .filter_map(|resource| resource.unstable_custom_metadata.as_ref())
        .find(|custom| custom.get("zenodoKey").and_then(Value::as_str) == Some("dataset.json"))
        .and_then(|custom| custom.get("zenodoUrl").and_then(Value::as_str))
        .map(String::from);

    let mut dataset = merge_datasets(&system_dataset, user_dataset_path.as_deref())?;

    for resource in dataset.resources.iter_mut().flatten() {
        resource.unstable_custom_metadata = None;
    }

    match serde_json::to_value(dataset) {
        Ok(Value::Object(descriptor)) => Ok(descriptor),
        _ => Err(Error::Parse("Dataset is not an object".to_string())),
    }
}
