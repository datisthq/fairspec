use std::fs;

use serde_json::{Value, json};

use crate::actions::descriptor::save::save_descriptor;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Error;

fn create_descriptor(value: Value) -> Descriptor {
    serde_json::from_value(value).unwrap()
}

fn read_json(path: &std::path::Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

#[test]
fn test_save_basic() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("output.json");
    save_descriptor(
        &create_descriptor(json!({"name": "test"})),
        path.to_str().unwrap(),
        false,
    )
    .unwrap();
    assert_eq!(read_json(&path), json!({"name": "test"}));
}

#[test]
fn test_save_creates_nested_dirs() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a").join("b").join("output.json");
    save_descriptor(
        &create_descriptor(json!({"name": "nested"})),
        path.to_str().unwrap(),
        false,
    )
    .unwrap();
    assert_eq!(read_json(&path), json!({"name": "nested"}));
}

#[test]
fn test_save_uses_two_space_indent() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("output.json");
    save_descriptor(
        &create_descriptor(json!({"a": 1})),
        path.to_str().unwrap(),
        false,
    )
    .unwrap();
    assert!(fs::read_to_string(&path).unwrap().contains("  \"a\": 1"));
}

#[test]
fn test_save_exclusive_fails_on_existing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("output.json");
    let path = path.to_str().unwrap();
    save_descriptor(&create_descriptor(json!({"a": 1})), path, false).unwrap();
    let error = save_descriptor(&create_descriptor(json!({"a": 2})), path, false).unwrap_err();
    assert!(matches!(error, Error::Io(error) if error.kind() == std::io::ErrorKind::AlreadyExists));
}

#[test]
fn test_save_overwrite_succeeds_on_existing() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("output.json");
    save_descriptor(
        &create_descriptor(json!({"a": 1})),
        path.to_str().unwrap(),
        false,
    )
    .unwrap();
    save_descriptor(
        &create_descriptor(json!({"a": 2})),
        path.to_str().unwrap(),
        true,
    )
    .unwrap();
    assert_eq!(read_json(&path), json!({"a": 2}));
}

#[test]
fn test_save_file_content_is_valid_json() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("output.json");
    let descriptor = json!({"key": "value", "nested": {"a": [1, 2, 3]}});
    save_descriptor(
        &create_descriptor(descriptor.clone()),
        path.to_str().unwrap(),
        false,
    )
    .unwrap();
    assert_eq!(read_json(&path), descriptor);
}
