use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::actions::dataset::save::save_dataset_descriptor;
use crate::actions::dataset::validate::validate_dataset_descriptor;
use crate::models::dataset::Dataset;
use crate::models::exception::Error;
use crate::settings::FAIRSPEC_VERSION;

fn create_dataset(value: Value) -> Dataset {
    serde_json::from_value(value).unwrap()
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn to_string(path: &Path) -> String {
    path.to_str().unwrap().to_string()
}

#[test]
fn test_saves_dataset() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dataset.json");
    let dataset = create_dataset(json!({
        "creators": [{"name": "Test Creator"}],
        "titles": [{"title": "Test Dataset"}],
        "resources": [{"name": "test_resource", "data": to_string(&dir.path().join("data.csv"))}],
    }));
    save_dataset_descriptor(&dataset, &to_string(&path), false).unwrap();
    let content = read_json(&path);
    assert!(
        content["$schema"]
            .as_str()
            .unwrap()
            .ends_with("dataset.json")
    );
    assert_eq!(content["creators"][0]["name"], "Test Creator");
    assert_eq!(content["resources"][0]["name"], "test_resource");
}

#[test]
fn test_sets_default_schema() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dataset.json");
    let dataset = create_dataset(json!({
        "resources": [{"data": to_string(&dir.path().join("data.csv"))}],
    }));
    save_dataset_descriptor(&dataset, &to_string(&path), false).unwrap();
    let expected = format!("https://fairspec.org/profiles/{FAIRSPEC_VERSION}/dataset.json");
    assert_eq!(read_json(&path)["$schema"], expected);
}

#[test]
fn test_preserves_custom_schema() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dataset.json");
    let dataset = create_dataset(json!({
        "$schema": "https://custom.schema.url/dataset.json",
        "resources": [{"data": to_string(&dir.path().join("data.csv"))}],
    }));
    save_dataset_descriptor(&dataset, &to_string(&path), false).unwrap();
    assert_eq!(
        read_json(&path)["$schema"],
        "https://custom.schema.url/dataset.json"
    );
}

#[test]
fn test_throws_when_file_exists() {
    let dir = tempfile::tempdir().unwrap();
    let path = to_string(&dir.path().join("dataset.json"));
    let dataset = create_dataset(json!({
        "resources": [{"data": to_string(&dir.path().join("data.csv"))}],
    }));
    save_dataset_descriptor(&dataset, &path, false).unwrap();
    let error = save_dataset_descriptor(&dataset, &path, false).unwrap_err();
    assert!(matches!(error, Error::Io(error) if error.kind() == std::io::ErrorKind::AlreadyExists));
}

#[test]
fn test_overwrites_when_flag_set() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dataset.json");
    let data = to_string(&dir.path().join("data.csv"));
    let dataset1 = create_dataset(json!({
        "creators": [{"name": "Initial"}],
        "resources": [{"data": data}],
    }));
    let dataset2 = create_dataset(json!({
        "creators": [{"name": "Updated"}],
        "resources": [{"data": data}],
    }));
    save_dataset_descriptor(&dataset1, &to_string(&path), false).unwrap();
    save_dataset_descriptor(&dataset2, &to_string(&path), true).unwrap();
    assert_eq!(read_json(&path)["creators"][0]["name"], "Updated");
}

#[test]
fn test_saves_to_nested_directory() {
    let dir = tempfile::tempdir().unwrap();
    let nested = dir.path().join("nested").join("dir");
    let path = nested.join("dataset.json");
    let dataset = create_dataset(json!({
        "resources": [{"data": to_string(&nested.join("data.csv"))}],
    }));
    save_dataset_descriptor(&dataset, &to_string(&path), false).unwrap();
    assert!(read_json(&path).get("resources").is_some());
}

#[test]
fn test_denormalizes_resource_paths() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dataset.json");
    let dataset = create_dataset(json!({
        "resources": [{"name": "test", "data": to_string(&dir.path().join("data.csv"))}],
    }));
    save_dataset_descriptor(&dataset, &to_string(&path), false).unwrap();
    assert_eq!(read_json(&path)["resources"][0]["data"], "data.csv");
}

#[test]
fn test_saved_dataset_validates_offline() {
    let dir = tempfile::tempdir().unwrap();
    let path = to_string(&dir.path().join("dataset.json"));
    let dataset = create_dataset(json!({
        "resources": [{
            "name": "test_resource",
            "data": to_string(&dir.path().join("data.csv")),
            "integrity": {"type": "sha256", "hash": "0000000000000000000000000000000000000000000000000000000000000000"},
        }],
    }));
    save_dataset_descriptor(&dataset, &path, false).unwrap();

    let report = validate_dataset_descriptor(path.as_str(), None).unwrap();

    assert_eq!(report.errors, vec![]);
    assert!(report.valid);
}
