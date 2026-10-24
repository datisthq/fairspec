use fairspec_metadata::{Descriptor, Error, Result};
use serde_json::{Value, json};

use super::from_ckan::convert_dataset_from_ckan;
use crate::actions::dataset::merge::merge_datasets;
use crate::plugins::ckan::models::dataset::CkanDataset;
use crate::plugins::ckan::models::schema::CkanSchema;
use crate::plugins::ckan::services::ckan::{dump_descriptor, make_ckan_api_request};

pub fn load_dataset_from_ckan(dataset_url: &str, api_key: Option<&str>) -> Result<Descriptor> {
    let dataset_id = extract_dataset_id(dataset_url).ok_or_else(|| {
        Error::Load(format!(
            "Failed to extract dataset ID from URL: {dataset_url}"
        ))
    })?;

    let mut ckan_dataset: CkanDataset = make_ckan_api_request(
        dataset_url,
        "package_show",
        &create_payload(json!({"id": dataset_id})),
        None,
        api_key,
    )?;

    for resource in &mut ckan_dataset.resources {
        let format = resource
            .format
            .as_deref()
            .unwrap_or_default()
            .to_uppercase();
        if ["CSV", "XLS", "XLSX"].contains(&format.as_str())
            && let Some(schema) = load_ckan_schema(dataset_url, &resource.id, api_key)
        {
            resource.schema = Some(schema);
        }
    }

    let system_dataset = convert_dataset_from_ckan(&ckan_dataset);
    let user_dataset_path = system_dataset
        .resources
        .iter()
        .flatten()
        .filter_map(|resource| resource.unstable_custom_metadata.as_ref())
        .find(|custom| custom.get("ckanKey").and_then(Value::as_str) == Some("dataset.json"))
        .and_then(|custom| custom.get("ckanUrl").and_then(Value::as_str))
        .map(String::from);

    let mut dataset = merge_datasets(&system_dataset, user_dataset_path.as_deref())?;

    for resource in dataset.resources.iter_mut().flatten() {
        resource.unstable_custom_metadata = None;
    }

    dump_descriptor(&dataset)
}

fn extract_dataset_id(dataset_url: &str) -> Option<String> {
    let parsed = url::Url::parse(dataset_url).ok()?;
    parsed
        .path()
        .split('/')
        .rfind(|part| !part.is_empty())
        .map(String::from)
}

fn load_ckan_schema(
    dataset_url: &str,
    resource_id: &str,
    api_key: Option<&str>,
) -> Option<CkanSchema> {
    let result: CkanSchema = make_ckan_api_request(
        dataset_url,
        "datastore_search",
        &create_payload(json!({"resource_id": resource_id, "limit": 0})),
        None,
        api_key,
    )
    .ok()?;
    let fields = result
        .fields
        .into_iter()
        .filter(|field| field.id != "_id" && field.id != "_full_text")
        .collect();
    Some(CkanSchema { fields })
}

fn create_payload(value: Value) -> Descriptor {
    match value {
        Value::Object(payload) => payload,
        _ => Descriptor::new(),
    }
}
