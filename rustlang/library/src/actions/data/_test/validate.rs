use fairspec_metadata::{ExpectedResourceType, FairspecError, Resource, ResourceError};
use serde_json::{Value, json};

use crate::actions::data::validate::validate_data;

fn create_resource(descriptor: Value) -> Resource {
    serde_json::from_value(descriptor).unwrap()
}

#[test]
fn test_should_return_valid_when_no_schema() {
    let resource = create_resource(json!({"data": [{"id": 1}]}));
    assert!(validate_data(&resource).unwrap().valid);
}

#[test]
fn test_should_validate_inline_data() {
    let resource = create_resource(json!({
        "data": {"name": "test", "age": 25},
        "dataSchema": {
            "type": "object",
            "properties": {"name": {"type": "string"}, "age": {"type": "integer"}},
            "required": ["name", "age"],
        },
    }));
    assert!(validate_data(&resource).unwrap().valid);
}

#[test]
fn test_should_detect_invalid_data() {
    let resource = create_resource(json!({
        "data": {"name": 123},
        "dataSchema": {"type": "object", "properties": {"name": {"type": "string"}}},
    }));
    let report = validate_data(&resource).unwrap();
    assert!(!report.valid);
    assert!(!report.errors.is_empty());
}

#[test]
fn test_should_return_error_when_no_data_but_schema_expected() {
    let resource = create_resource(json!({"dataSchema": {"type": "object"}}));
    let report = validate_data(&resource).unwrap();
    assert!(!report.valid);
    assert_eq!(report.errors.len(), 1);
    let FairspecError::Resource(ResourceError::Type(error)) = &report.errors[0] else {
        panic!("expected a resource/type error, got {:?}", report.errors[0]);
    };
    assert_eq!(error.expected_resource_type, ExpectedResourceType::Data);
}

#[test]
fn test_should_validate_empty_inline_object() {
    let resource = create_resource(json!({
        "data": {},
        "dataSchema": {"type": "object", "required": ["name"]},
    }));
    let report = validate_data(&resource).unwrap();
    assert!(!report.valid);
    assert_eq!(report.errors.len(), 1);
}
