use serde_json::{Value, json};

use crate::actions::json::inspect::JsonIssue;
use crate::actions::json_schema::inspect::inspect_json_schema;

fn inspect(value: Value) -> Vec<JsonIssue> {
    inspect_json_schema(&serde_json::from_value(value).unwrap(), None)
}

#[test]
fn test_valid_empty_object_schema() {
    assert_eq!(inspect(json!({"type": "object"})), vec![]);
}

#[test]
fn test_valid_string_schema() {
    assert_eq!(inspect(json!({"type": "string"})), vec![]);
}

#[test]
fn test_valid_schema_with_properties() {
    let schema = json!({"type": "object", "properties": {"name": {"type": "string"}}});
    assert_eq!(inspect(schema), vec![]);
}

#[test]
fn test_valid_schema_with_required() {
    let schema = json!({
        "type": "object",
        "required": ["name"],
        "properties": {"name": {"type": "string"}},
    });
    assert_eq!(inspect(schema), vec![]);
}

#[test]
fn test_valid_array_schema() {
    assert_eq!(
        inspect(json!({"type": "array", "items": {"type": "integer"}})),
        vec![]
    );
}

#[test]
fn test_valid_draft_2020_12_schema() {
    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
    });
    assert_eq!(inspect(schema), vec![]);
}

#[test]
fn test_invalid_type_value() {
    let issues = inspect(json!({"type": "invalid"}));
    assert!(!issues.is_empty());
    assert_eq!(issues[0].json_pointer, "/type");
}

#[test]
fn test_invalid_required_not_array() {
    assert!(!inspect(json!({"required": "name"})).is_empty());
}

#[test]
fn test_invalid_properties_not_object() {
    assert!(!inspect(json!({"properties": "invalid"})).is_empty());
}

#[test]
fn test_root_json_pointer_combined() {
    let descriptor = serde_json::from_value(json!({"type": "invalid"})).unwrap();
    let issues = inspect_json_schema(&descriptor, Some("/root"));
    assert!(!issues.is_empty());
    assert_eq!(issues[0].json_pointer, "/root/type");
}
