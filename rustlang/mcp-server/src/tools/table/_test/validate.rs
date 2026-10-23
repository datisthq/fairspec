use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::{Value, json};

use crate::tools::table::validate::{ValidateTableInput, ValidateTableResult, validate_table_tool};

fn validate(input: Value) -> ValidateTableResult {
    let input: ValidateTableInput = serde_json::from_value(input).unwrap();
    validate_table_tool(input).unwrap()
}

fn write_csv(text: &str) -> String {
    write_temp_file(text, false, None, Some("csv")).unwrap()
}

fn create_age_schema() -> Value {
    json!({"properties": {"age": {"type": "integer", "maximum": 150}}})
}

#[test]
fn test_should_validate_valid_table() {
    let result = validate(json!({"resource": {"data": write_csv("id,age\n1,25\n2,30\n")}}));
    assert!(result.report.valid);
}

#[test]
fn test_should_report_rows_failing_a_check() {
    let result = validate(json!({
        "resource": {"data": [{"start": 1, "end": 2}, {"start": 3, "end": 2}]},
        "options": {"checks": [{"name": "end-after-start", "condition": "\"end\" >= start"}]},
    }));
    assert_eq!(
        serde_json::to_value(&result.report.errors).unwrap(),
        json!([{"type": "row/check", "checkName": "end-after-start", "rowNumber": 2}])
    );
}

#[test]
fn test_should_detect_invalid_table() {
    let path = write_csv("id,age\n1,200\n2,300\n");
    let result = validate(json!({"resource": {"data": path, "tableSchema": create_age_schema()}}));
    assert!(!result.report.valid);
    assert_eq!(result.report.errors.len(), 2);
}

#[test]
fn test_should_forward_options() {
    let path = write_csv("id,age\n1,200\n2,300\n");
    let result = validate(json!({
        "resource": {"data": path, "tableSchema": create_age_schema()},
        "options": {"maxErrors": 1},
    }));
    assert_eq!(result.report.errors.len(), 1);
}

#[test]
fn test_should_return_at_most_100_errors_by_default() {
    let data: Vec<Value> = (0..300).map(|_| json!({"age": "invalid"})).collect();
    let result = validate(json!({
        "resource": {"data": data, "tableSchema": {"properties": {"age": {"type": "integer"}}}},
    }));
    assert!(!result.report.valid);
    assert_eq!(result.report.errors.len(), 100);
    assert!(result.truncated);
}

#[test]
fn test_should_respect_max_errors() {
    let data: Vec<Value> = (0..10).map(|_| json!({"age": "invalid"})).collect();
    let result = validate(json!({
        "resource": {"data": data, "tableSchema": {"properties": {"age": {"type": "integer"}}}},
        "maxErrors": 3,
    }));
    assert_eq!(result.report.errors.len(), 3);
    assert!(result.truncated);
}

#[test]
fn test_should_not_mark_complete_report_as_truncated() {
    let path = write_csv("id,age\n1,200\n");
    let result = validate(json!({"resource": {"data": path, "tableSchema": create_age_schema()}}));
    assert_eq!(result.report.errors.len(), 1);
    assert!(!result.truncated);
}

#[test]
fn test_should_fill_max_errors_from_one_bad_column_of_many() {
    let data: Vec<Value> = (0..300)
        .map(|i| json!({"id": i, "age": "invalid"}))
        .collect();
    let result = validate(json!({
        "resource": {
            "data": data,
            "tableSchema": {"properties": {"id": {"type": "integer"}, "age": {"type": "integer"}}},
        },
    }));
    assert_eq!(result.report.errors.len(), 100);
    assert!(result.truncated);
}
