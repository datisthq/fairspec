use serde_json::{Value, json};

use crate::actions::dataset::validate::validate_dataset_descriptor;
use crate::models::dataset::DatasetValidationResult;
use crate::models::descriptor::Descriptor;

fn validate(value: Value) -> DatasetValidationResult {
    let descriptor: Descriptor = serde_json::from_value(value).unwrap();
    validate_dataset_descriptor(descriptor, None).unwrap()
}

#[test]
fn test_valid_dataset() {
    let result = validate(json!({"resources": [{"data": "data.csv"}]}));
    assert!(result.valid);
    assert_eq!(result.errors, vec![]);
}

#[test]
fn test_invalid_dataset() {
    let result = validate(json!({"resources": "not-an-array"}));
    assert!(!result.valid);
    assert!(!result.errors.is_empty());
}

#[test]
fn test_missing_schema_is_valid() {
    let result = validate(json!({"resources": [{"data": "data.csv"}]}));
    assert!(result.valid);
}

#[test]
fn test_dataset_with_datacite() {
    let result = validate(json!({
        "creators": [{"name": "John Doe"}],
        "titles": [{"title": "Example Dataset"}],
        "resources": [{"data": "data.csv"}],
    }));
    assert!(result.valid);
    assert_eq!(result.errors, vec![]);
}
