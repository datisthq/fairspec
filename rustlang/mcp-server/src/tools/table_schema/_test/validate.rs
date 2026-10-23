use fairspec_library::fairspec_dataset::write_temp_file;
use fairspec_library::fairspec_metadata::Report;
use serde_json::{Value, json};

use crate::tools::table_schema::validate::{
    ValidateTableSchemaInput, ValidateTableSchemaResult, validate_table_schema_tool,
};

fn validate(source: Value) -> ValidateTableSchemaResult {
    let input: ValidateTableSchemaInput =
        serde_json::from_value(json!({"source": source})).unwrap();
    validate_table_schema_tool(input).unwrap()
}

#[test]
fn test_should_validate_valid_descriptor() {
    let result = validate(json!({"properties": {"id": {"type": "integer"}}}));
    assert!(result.report.valid);
    assert!(result.report.errors.is_empty());
}

#[test]
fn test_should_detect_invalid_descriptor() {
    let result = validate(json!({"properties": {"id": {"type": 123}}}));
    assert!(!result.report.valid);
    assert!(!result.report.errors.is_empty());
}

#[test]
fn test_should_validate_descriptor_from_path() {
    let path = write_temp_file(
        r#"{"properties": {"id": {"type": "integer"}}}"#,
        false,
        None,
        Some("json"),
    )
    .unwrap();
    assert!(validate(json!(path)).report.valid);
}

#[test]
fn test_should_return_plain_report() {
    let report: Report = validate(json!({"properties": {}})).report;
    assert_eq!(
        serde_json::to_value(report).unwrap(),
        json!({"valid": true, "errors": []})
    );
}
