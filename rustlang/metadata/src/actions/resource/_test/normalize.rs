use serde_json::json;

use crate::actions::dataset::denormalize::denormalize_dataset;
use crate::actions::dataset::normalize::normalize_dataset;
use crate::actions::resource::denormalize::denormalize_resource;
use crate::actions::resource::normalize::normalize_resource;
use crate::models::dataset::Dataset;
use crate::models::resource::Resource;

#[test]
fn test_normalize_resource_paths() {
    let resource: Resource = serde_json::from_value(json!({
        "data": ["a.csv", {"x": 1}],
        "fileDialect": "dialect.json",
        "tableSchema": {"properties": {}},
        "dataSchema": "https://example.com/schema.json",
    }))
    .unwrap();
    let descriptor = normalize_resource(&resource, Some("folder")).unwrap();
    assert_eq!(
        serde_json::Value::Object(descriptor),
        json!({
            "data": ["folder/a.csv", {"x": 1}],
            "fileDialect": "folder/dialect.json",
            "tableSchema": {"properties": {}},
            "dataSchema": "https://example.com/schema.json",
        })
    );
}

#[test]
fn test_denormalize_resource_paths() {
    let resource: Resource = serde_json::from_value(
        json!({"data": "folder/a.csv", "tableSchema": "folder/schema.json"}),
    )
    .unwrap();
    let descriptor = denormalize_resource(&resource, Some("folder")).unwrap();
    assert_eq!(
        serde_json::Value::Object(descriptor),
        json!({"data": "a.csv", "tableSchema": "schema.json"})
    );
}

#[test]
fn test_normalize_dataset_outside_cwd() {
    let dataset: Dataset = serde_json::from_value(
        json!({"resources": [{"data": "a.csv", "tableSchema": "schema.json"}]}),
    )
    .unwrap();
    let normalized = normalize_dataset(&dataset, Some("..")).unwrap();
    let resource = &normalized.resources.as_ref().unwrap()[0];
    assert_eq!(resource.data, Some(json!("../a.csv")));

    let denormalized = denormalize_dataset(&normalized, Some("..")).unwrap();
    assert_eq!(denormalized, dataset);
}

#[test]
fn test_normalize_rejects_an_escaping_path() {
    let dataset: Dataset =
        serde_json::from_value(json!({"resources": [{"data": "../a.csv"}]})).unwrap();
    assert!(normalize_dataset(&dataset, Some("folder")).is_err());
}
