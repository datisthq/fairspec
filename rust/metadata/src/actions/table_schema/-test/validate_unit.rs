use serde_json::{Value, json};

use crate::actions::table_schema::validate::validate_table_schema;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use crate::models::table_schema::TableSchemaValidationResult;

fn validate(value: Value) -> Result<TableSchemaValidationResult> {
    let descriptor: Descriptor = serde_json::from_value(value).unwrap();
    validate_table_schema(descriptor, None)
}

#[test]
fn test_valid_schema() {
    let result = validate(json!({
        "properties": {"id": {"type": "integer"}, "name": {"type": "string"}},
    }))
    .unwrap();
    assert!(result.valid);
    assert_eq!(result.errors, vec![]);
}

#[test]
fn test_invalid_schema() {
    let result = validate(json!({"properties": {"id": {"type": 123}}})).unwrap();
    assert!(!result.valid);
    assert!(!result.errors.is_empty());
}

#[test]
fn test_missing_schema_is_valid() {
    let result = validate(json!({"properties": {"id": {"type": "integer"}}})).unwrap();
    assert!(result.valid);
}

#[test]
fn test_wrong_profile_type_raises() {
    let error = validate(json!({
        "$schema": "https://fairspec.org/profiles/latest/dataset.json",
        "properties": {"id": {"type": "integer"}},
    }))
    .unwrap_err();
    assert!(error.to_string().contains("table-schema"));
}
