use serde_json::json;

use crate::_test::{run, write_file};

#[test]
fn test_should_keep_zero_exit_code_for_valid_table() {
    let folder = tempfile::tempdir().unwrap();
    let data = write_file(folder.path(), "data.csv", "id,age\n1,25\n2,30\n");
    let schema = write_file(
        folder.path(),
        "schema.json",
        &json!({"properties": {"age": {"type": "integer"}}}).to_string(),
    );

    let result = run(&["table", "validate", &data, "--schema", &schema, "--json"]);

    assert_eq!(result.code, 0, "{}{}", result.stdout, result.stderr);
    assert_eq!(result.json()["valid"], true);
}

#[test]
fn test_should_set_non_zero_exit_code_for_invalid_table() {
    let folder = tempfile::tempdir().unwrap();
    let data = write_file(folder.path(), "data.csv", "id,age\n1,200\n");
    let schema = write_file(
        folder.path(),
        "schema.json",
        &json!({"properties": {"age": {"type": "integer", "maximum": 150}}}).to_string(),
    );

    let result = run(&["table", "validate", &data, "--schema", &schema, "--json"]);

    assert_ne!(result.code, 0);
    assert_eq!(result.json()["valid"], false);
}

#[test]
fn test_should_apply_column_properties_without_a_schema() {
    let folder = tempfile::tempdir().unwrap();
    let data = write_file(folder.path(), "data.csv", "id,age\n1,200\n");

    let result = run(&[
        "table",
        "validate",
        &data,
        "--column-properties",
        r#"{"age": {"type": "integer", "maximum": 150}}"#,
        "--json",
    ]);

    assert_ne!(result.code, 0);
    assert_eq!(result.json()["valid"], false);
}

#[test]
fn test_should_report_rows_failing_a_check() {
    let folder = tempfile::tempdir().unwrap();
    let data = write_file(folder.path(), "data.csv", "id,start,end\n1,1,2\n2,3,2");

    let result = run(&[
        "table",
        "validate",
        &data,
        "--check",
        r#"end-after-start="end" >= start"#,
        "--check",
        "two-rows=count(*) = 2",
        "--json",
    ]);

    assert_ne!(result.code, 0);
    assert_eq!(
        result.json()["errors"],
        json!([{"type": "row/check", "checkName": "end-after-start", "rowNumber": 2}])
    );
}

#[test]
fn test_should_reject_a_check_without_a_name() {
    let result = run(&["table", "validate", "data.csv", "--check", "=count(*) = 2"]);

    assert_eq!(result.code, 1);
    assert!(
        result.stderr.contains("name=condition"),
        "{}",
        result.stderr
    );
}
