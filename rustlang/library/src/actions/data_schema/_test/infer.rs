use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::actions::data_schema::infer::infer_data_schema;

fn infer(descriptor: Value) -> Option<Value> {
    let resource: Resource = serde_json::from_value(descriptor).unwrap();
    infer_data_schema(&resource).unwrap().map(Value::Object)
}

#[test]
fn test_should_infer_schema_from_inline_data() {
    assert_eq!(
        infer(json!({"data": {"name": "test", "age": 25, "active": true}})),
        Some(json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer"},
                "active": {"type": "boolean"},
            },
        }))
    );
}

#[test]
fn test_should_infer_schema_from_inline_array() {
    assert_eq!(
        infer(json!({"data": [{"id": 1}, {"id": 2}]})),
        Some(json!({
            "type": "array",
            "items": {"type": "object", "properties": {"id": {"type": "integer"}}},
        }))
    );
}

#[test]
fn test_should_leave_items_out_when_items_differ() {
    assert_eq!(
        infer(json!({"data": [{"a": 1}, {"a": "x"}]})),
        Some(json!({"type": "array"}))
    );
}

#[test]
fn test_should_infer_empty_object() {
    assert_eq!(infer(json!({"data": {}})), Some(json!({"type": "object"})));
}

#[test]
fn test_should_infer_schema_from_json_file() {
    let path = write_temp_file(
        r#"{"name": "test", "value": 42}"#,
        false,
        None,
        Some("json"),
    )
    .unwrap();
    let schema = infer(json!({"data": path})).unwrap();
    assert_eq!(schema["type"], "object");
}

#[test]
fn test_should_return_none_for_no_data() {
    assert_eq!(infer(json!({})), None);
}

#[test]
fn test_should_return_none_for_csv_file() {
    let path = write_temp_file("id,name\n1,english", false, None, Some("csv")).unwrap();
    assert_eq!(infer(json!({"data": path})), None);
}
