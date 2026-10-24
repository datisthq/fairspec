use std::fs;

use fairspec_dataset::write_temp_file;
use fairspec_metadata::{Dataset, validate_dataset_descriptor};
use fairspec_table::InferTableSchemaOptions;
use serde_json::{Value, json};

use crate::actions::dataset::infer::infer_dataset;

fn write_csv(content: &str) -> String {
    write_temp_file(content, false, None, Some("csv")).unwrap()
}

fn create_dataset(descriptor: Value) -> Dataset {
    serde_json::from_value(descriptor).unwrap()
}

#[test]
fn test_should_infer_resource_names() {
    let dataset = create_dataset(json!({"resources": [{"data": write_csv("id,name\n1,english")}]}));
    let result = infer_dataset(&dataset, &Default::default()).unwrap();
    let resources = result.resources.unwrap();
    assert_eq!(resources.len(), 1);
    assert!(resources[0].name.is_some());
}

#[test]
fn test_should_not_mutate_original() {
    let dataset = create_dataset(json!({"resources": [{"data": write_csv("id,name\n1,english")}]}));
    let result = infer_dataset(&dataset, &Default::default()).unwrap();
    assert_ne!(result, dataset);
    assert!(dataset.resources.unwrap()[0].name.is_none());
}

#[test]
fn test_should_handle_empty_resources() {
    let dataset = create_dataset(json!({"resources": []}));
    let result = infer_dataset(&dataset, &Default::default()).unwrap();
    assert_eq!(result.resources, Some(Vec::new()));
}

#[test]
fn test_should_handle_no_resources() {
    let result = infer_dataset(&Dataset::default(), &Default::default()).unwrap();
    assert_eq!(result.resources, None);
}

#[test]
fn test_should_pass_table_schema_options() {
    let dataset = create_dataset(json!({"resources": [{"data": write_csv("id,name\n1,english")}]}));
    let options = InferTableSchemaOptions {
        keep_strings: Some(true),
        ..Default::default()
    };
    let result = infer_dataset(&dataset, &options).unwrap();
    let resource = serde_json::to_value(&result.resources.unwrap()[0]).unwrap();
    assert_eq!(
        resource["tableSchema"]["properties"]["id"]["type"],
        "string"
    );
}

// The only test that changes the working directory; every other test uses absolute paths
#[test]
fn test_should_round_trip_inferred_dataset_through_validation() {
    let folder = tempfile::tempdir().unwrap();
    fs::write(
        folder.path().join("products.csv"),
        "id,name\n1,english\n2,spanish\n",
    )
    .unwrap();
    let cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(folder.path()).unwrap();

    let dataset = create_dataset(json!({"resources": [{"data": "products.csv"}]}));
    let inferred = infer_dataset(&dataset, &Default::default());
    std::env::set_current_dir(cwd).unwrap();

    let descriptor = match serde_json::to_value(inferred.unwrap()).unwrap() {
        Value::Object(descriptor) => descriptor,
        _ => unreachable!(),
    };
    let result = validate_dataset_descriptor(descriptor, None).unwrap();
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_should_infer_resources_in_order_under_concurrency() {
    let paths: Vec<String> = (0..4)
        .map(|index| write_csv(&format!("id,name\n{index},english")))
        .collect();
    let resources: Vec<Value> = paths.iter().map(|path| json!({"data": path})).collect();
    let dataset = create_dataset(json!({"resources": resources}));

    let infer = |concurrency| {
        let options = InferTableSchemaOptions {
            concurrency: Some(concurrency),
            ..Default::default()
        };
        infer_dataset(&dataset, &options).unwrap()
    };
    let serial = infer(1);
    let concurrent = infer(4);
    assert_eq!(serial, concurrent);

    let data: Vec<Value> = serial
        .resources
        .unwrap()
        .iter()
        .map(|resource| serde_json::to_value(&resource.data).unwrap())
        .collect();
    let expected: Vec<Value> = paths.into_iter().map(Value::String).collect();
    assert_eq!(data, expected);
}
