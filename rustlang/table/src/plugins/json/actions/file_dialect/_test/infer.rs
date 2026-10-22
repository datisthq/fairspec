use fairspec_dataset::write_temp_file;
use fairspec_metadata::{FileDialect, Resource};
use serde_json::{Value, json};

use crate::plugins::json::actions::file_dialect::infer::infer_json_file_dialect;

fn infer_file(body: &str, format: &str) -> Option<FileDialect> {
    let path = write_temp_file(body, false, None, None).unwrap();
    infer(json!({"data": path, "fileDialect": {"format": format}}))
}

fn infer(resource: Value) -> Option<FileDialect> {
    let resource: Resource = serde_json::from_value(resource).unwrap();
    infer_json_file_dialect(&resource, None).unwrap()
}

fn create_dialect(value: Value) -> Option<FileDialect> {
    Some(serde_json::from_value(value).unwrap())
}

#[test]
fn test_should_detect_row_type_as_object_without_header_rows() {
    let result = infer_file(
        r#"[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]"#,
        "json",
    );
    assert_eq!(
        result,
        create_dialect(json!({"format": "json", "rowType": "object"}))
    );
}

#[test]
fn test_should_detect_headers_when_first_row_is_header() {
    let result = infer_file(r#"[["id", "name"], [1, "Alice"], [2, "Bob"]]"#, "json");
    assert_eq!(
        result,
        create_dialect(json!({"format": "json", "rowType": "array", "headerRows": [1]}))
    );
}

#[test]
fn test_should_detect_no_headers_when_data_rows_only() {
    let result = infer_file("[[1, 2, 3], [4, 5, 6], [7, 8, 9]]", "json");
    assert_eq!(
        result,
        create_dialect(json!({"format": "json", "rowType": "array", "headerRows": false}))
    );
}

#[test]
fn test_should_detect_json_pointer_for_nested_data_structure() {
    let body =
        r#"{"metadata": {"version": "1.0"}, "data": [["id", "name"], [1, "Alice"], [2, "Bob"]]}"#;
    let Some(FileDialect::Json(result)) = infer_file(body, "json") else {
        panic!("expected a json dialect");
    };
    assert_eq!(result.json_pointer.as_deref(), Some("/data"));
    assert_eq!(result.row_type, Some(fairspec_metadata::RowType::Array));
}

#[test]
fn test_should_detect_json_pointer_with_array_of_objects() {
    let body = r#"{"metadata": {"version": "1.0"}, "records": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}"#;
    assert_eq!(
        infer_file(body, "json"),
        create_dialect(json!({"format": "json", "jsonPointer": "/records", "rowType": "object"}))
    );
}

#[test]
fn test_should_detect_row_type_for_array_jsonl() {
    let body = "[1, \"Alice\", 30]\n[2, \"Bob\", 25]\n[3, \"Charlie\", 35]";
    assert_eq!(
        infer_file(body, "jsonl"),
        create_dialect(json!({"format": "jsonl", "rowType": "array", "headerRows": false}))
    );
}

#[test]
fn test_should_detect_row_type_for_object_jsonl() {
    let body = "{\"id\": 1, \"name\": \"Alice\"}\n{\"id\": 2, \"name\": \"Bob\"}\n{\"id\": 3, \"name\": \"Charlie\"}";
    assert_eq!(
        infer_file(body, "jsonl"),
        create_dialect(json!({"format": "jsonl", "rowType": "object"}))
    );
}

#[test]
fn test_should_return_format_only_for_empty_array() {
    assert_eq!(
        infer_file("[]", "json"),
        create_dialect(json!({"format": "json"}))
    );
}

#[test]
fn test_should_return_format_only_for_invalid_json() {
    assert_eq!(
        infer_file("{this is not valid json", "json"),
        create_dialect(json!({"format": "json"}))
    );
}

#[test]
fn test_should_return_none_for_inline_data() {
    let resource = json!({
        "data": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}],
        "fileDialect": {"format": "json"},
    });
    assert_eq!(infer(resource), None);
}

#[test]
fn test_should_return_none_for_unsupported_format() {
    assert_eq!(infer_file("id,name\n1,Alice\n2,Bob", "csv"), None);
}

#[test]
fn test_should_return_format_only_for_single_row_array() {
    assert_eq!(
        infer_file("[[1, 2, 3]]", "json"),
        create_dialect(json!({"format": "json", "rowType": "array", "headerRows": false}))
    );
}

#[test]
fn test_should_handle_non_file_path_errors_gracefully() {
    let resource =
        json!({"data": "/nonexistent/path/to/file.json", "fileDialect": {"format": "json"}});
    assert_eq!(infer(resource), create_dialect(json!({"format": "json"})));
}
