use serde_json::{Value, json};

use crate::tools::data::validate::{ValidateDataInput, ValidateDataResult, validate_data_tool};

fn validate(data: Value) -> ValidateDataResult {
    let input: ValidateDataInput = serde_json::from_value(json!({"resource": {
        "data": data,
        "dataSchema": {"type": "object", "properties": {"name": {"type": "string"}}},
    }}))
    .unwrap();
    validate_data_tool(input).unwrap()
}

#[test]
fn test_should_validate_data_against_schema() {
    assert!(validate(json!({"name": "alice"})).report.valid);
}

#[test]
fn test_should_detect_invalid_data() {
    let result = validate(json!({"name": 1}));
    assert!(!result.report.valid);
    assert!(!result.report.errors.is_empty());
}
