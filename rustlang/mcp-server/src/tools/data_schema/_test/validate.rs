use fairspec_library::fairspec_dataset::write_temp_file;
use fairspec_library::fairspec_metadata::Report;
use serde_json::{Value, json};

use crate::tools::data_schema::validate::{
    ValidateDataSchemaInput, ValidateDataSchemaResult, validate_data_schema_tool,
};

fn validate(source: Value) -> ValidateDataSchemaResult {
    let input: ValidateDataSchemaInput = serde_json::from_value(json!({"source": source})).unwrap();
    validate_data_schema_tool(input).unwrap()
}

#[test]
fn test_should_validate_valid_descriptor() {
    let result = validate(json!({"type": "object"}));
    assert!(result.report.valid);
    assert!(result.report.errors.is_empty());
}

#[test]
fn test_should_detect_invalid_descriptor() {
    let result = validate(json!({"type": 123}));
    assert!(!result.report.valid);
    assert!(!result.report.errors.is_empty());
}

#[test]
fn test_should_validate_descriptor_from_path() {
    let path = write_temp_file(r#"{"type": "object"}"#, false, None, Some("json")).unwrap();
    assert!(validate(json!(path)).report.valid);
}

#[test]
fn test_should_return_plain_report() {
    let report: Report = validate(json!({"type": "object"})).report;
    assert_eq!(
        serde_json::to_value(report).unwrap(),
        json!({"valid": true, "errors": []})
    );
}
