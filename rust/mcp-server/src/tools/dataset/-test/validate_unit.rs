use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::{Value, json};

use crate::tools::dataset::validate::{
    ValidateDatasetInput, ValidateDatasetResult, validate_dataset_tool,
};

fn validate(source: Value) -> ValidateDatasetResult {
    let input: ValidateDatasetInput = serde_json::from_value(json!({"source": source})).unwrap();
    validate_dataset_tool(input).unwrap()
}

fn create_dataset(data: Value) -> Value {
    json!({"resources": [{
        "name": "people",
        "data": data,
        "tableSchema": {"properties": {"id": {"type": "integer"}}},
    }]})
}

#[test]
fn test_should_validate_valid_dataset() {
    let result = validate(create_dataset(json!([{"id": 1}, {"id": 2}])));
    assert!(result.report.valid);
    assert!(result.report.errors.is_empty());
}

#[test]
fn test_should_detect_invalid_resource_data() {
    let result = validate(create_dataset(json!([{"id": "one"}])));
    assert!(!result.report.valid);
    assert!(!result.report.errors.is_empty());
}

#[test]
fn test_should_validate_dataset_from_path() {
    let text = r#"{"resources": [{"name": "people", "data": [{"id": 1}]}]}"#;
    let path = write_temp_file(text, false, None, Some("json")).unwrap();
    assert!(validate(json!(path)).report.valid);
}
