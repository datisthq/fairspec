use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::actions::resource::infer::infer_resource;

fn create_csv_resource(extra: Value) -> Resource {
    let path = write_temp_file("id,name\n1,english", false, None, Some("csv")).unwrap();
    let mut descriptor = json!({"data": path});
    for (key, value) in extra.as_object().unwrap() {
        descriptor[key] = value.clone();
    }
    serde_json::from_value(descriptor).unwrap()
}

#[test]
fn test_should_infer_name() {
    let result = infer_resource(&create_csv_resource(json!({})), None, &Default::default());
    assert!(result.unwrap().name.is_some());
}

#[test]
fn test_should_infer_file_dialect() {
    let result = infer_resource(&create_csv_resource(json!({})), None, &Default::default());
    assert!(result.unwrap().file_dialect.is_some());
}

#[test]
fn test_should_not_overwrite_existing_name() {
    let resource = create_csv_resource(json!({"name": "custom"}));
    let result = infer_resource(&resource, None, &Default::default()).unwrap();
    assert_eq!(result.name.as_deref(), Some("custom"));
}

#[test]
fn test_should_not_mutate_original() {
    let resource = create_csv_resource(json!({}));
    let result = infer_resource(&resource, None, &Default::default()).unwrap();
    assert_ne!(result, resource);
    assert!(resource.name.is_none());
}

#[test]
fn test_should_handle_resource_number() {
    let resource: Resource = serde_json::from_value(json!({"data": [{"id": 1}]})).unwrap();
    let result = infer_resource(&resource, Some(5), &Default::default()).unwrap();
    assert_eq!(result.name.as_deref(), Some("resource5"));
}
