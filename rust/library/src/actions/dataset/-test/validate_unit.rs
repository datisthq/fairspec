use fairspec_metadata::{Dataset, Descriptor};
use serde_json::{Value, json};

use crate::actions::dataset::validate::validate_dataset;
use crate::models::table::ValidateTableOptions;

fn create_schema() -> Value {
    json!({"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}})
}

fn create_dataset(resources: Value) -> Dataset {
    serde_json::from_value(json!({"resources": resources})).unwrap()
}

fn create_descriptor(value: Value) -> Descriptor {
    match value {
        Value::Object(descriptor) => descriptor,
        _ => unreachable!(),
    }
}

#[test]
fn test_should_validate_valid_dataset() {
    let dataset = create_dataset(json!([{
        "data": [{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}],
        "name": "test",
        "tableSchema": create_schema(),
    }]));
    assert!(
        validate_dataset(dataset, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_detect_invalid_resource() {
    let dataset = create_dataset(json!([{
        "data": [{"id": "BAD", "name": "english"}],
        "name": "test",
        "tableSchema": create_schema(),
    }]));
    assert!(
        !validate_dataset(dataset, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_handle_empty_dataset() {
    let dataset = create_dataset(json!([]));
    assert!(
        validate_dataset(dataset, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_return_invalid_report_for_bad_descriptor() {
    let descriptor = create_descriptor(json!({"resources": "bad"}));
    let report = validate_dataset(descriptor, &Default::default()).unwrap();
    assert!(!report.valid);
    assert!(!report.errors.is_empty());
}

#[test]
fn test_should_validate_valid_descriptor() {
    let descriptor = create_descriptor(json!({
        "resources": [{
            "name": "test",
            "data": [{"id": 1, "name": "english"}],
            "tableSchema": create_schema(),
        }],
    }));
    assert!(
        validate_dataset(descriptor, &Default::default())
            .unwrap()
            .valid
    );
}

fn create_concurrency_dataset() -> Dataset {
    create_dataset(json!([
        {"data": [{"id": "BAD", "name": "english"}, {"id": 2, "name": "中文"}], "tableSchema": create_schema()},
        {"data": [{"id": 1, "name": "english"}], "tableSchema": create_schema()},
        {"data": [{"id": "ALSO BAD", "name": "français"}], "tableSchema": create_schema()},
    ]))
}

fn create_options(concurrency: usize) -> ValidateTableOptions {
    serde_json::from_value(json!({"concurrency": concurrency})).unwrap()
}

#[test]
fn test_should_produce_identical_report_for_serial_and_concurrent() {
    let serial = validate_dataset(create_concurrency_dataset(), &create_options(1)).unwrap();
    let concurrent = validate_dataset(create_concurrency_dataset(), &create_options(4)).unwrap();
    assert_eq!(serial, concurrent);
}

#[test]
fn test_should_assign_resource_names_before_validating() {
    let report = validate_dataset(create_concurrency_dataset(), &create_options(4)).unwrap();
    assert!(!report.valid);
    let names: Vec<Value> = serde_json::to_value(&report.errors)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|error| error["resourceName"].clone())
        .collect();
    assert_eq!(names, [json!("resource1"), json!("resource3")]);
}

#[test]
fn test_should_validate_frictionless_descriptor_with_package_names() {
    let descriptor = create_descriptor(json!({
        "resources": [{
            "name": "fruit-list",
            "data": [["id"], ["x"]],
            "schema": {"fields": [{"name": "id", "type": "integer"}]},
        }],
    }));
    let report = validate_dataset(descriptor, &Default::default()).unwrap();
    assert!(!report.valid);
    let errors = serde_json::to_value(report.errors).unwrap();
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(errors[0]["resourceName"], "fruit-list");
}

#[test]
fn test_should_point_frictionless_descriptor_errors_at_package() {
    let descriptor = create_descriptor(json!({
        "profile": "data-package",
        "resources": [{"name": "a-b", "data": []}, {"name": "a_b", "data": []}],
    }));
    let report = validate_dataset(descriptor, &Default::default()).unwrap();
    let errors = serde_json::to_value(report.errors).unwrap();
    assert_eq!(errors[0]["jsonPointer"], "/resources/1/name");
}
