use serde_json::{Value, json};

use crate::actions::json::inspect::{JsonIssue, inspect_json};
use crate::models::json_schema::JsonSchema;

fn create_schema() -> JsonSchema {
    to_map(json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer"},
            "address": {
                "type": "object",
                "properties": {"city": {"type": "string"}},
                "required": ["city"],
            },
        },
        "required": ["name"],
    }))
}

fn to_map(value: Value) -> JsonSchema {
    serde_json::from_value(value).unwrap()
}

fn inspect(value: Value) -> Vec<JsonIssue> {
    inspect_json(&value, create_schema(), None).unwrap()
}

#[test]
fn test_valid_value() {
    assert_eq!(inspect(json!({"name": "test"})), vec![]);
}

#[test]
fn test_type_mismatch() {
    let issues = inspect(json!({"name": 123}));
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].json_pointer, "/name");
}

#[test]
fn test_missing_required() {
    let issues = inspect(json!({}));
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("name"));
    assert_eq!(issues[0].json_pointer, "");
}

#[test]
fn test_escapes_reference_tokens() {
    let schema = to_map(json!({
        "properties": {"a/b": {"type": "integer"}, "c~d": {"type": "integer"}}
    }));
    let issues = inspect_json(&json!({"a/b": "x", "c~d": "x"}), schema, None).unwrap();
    let mut pointers: Vec<_> = issues.into_iter().map(|issue| issue.json_pointer).collect();
    pointers.sort();
    assert_eq!(pointers, vec!["/a~1b", "/c~0d"]);
}

#[test]
fn test_nested_validation() {
    let issues = inspect(json!({"name": "test", "address": {}}));
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].json_pointer, "/address");
}

#[test]
fn test_multiple_errors() {
    assert_eq!(
        inspect(json!({"name": 123, "age": "not_a_number"})).len(),
        2
    );
}

#[test]
fn test_root_json_pointer_combined() {
    let issues = inspect_json(&json!({"name": 123}), create_schema(), Some("/root")).unwrap();
    assert_eq!(issues[0].json_pointer, "/root/name");
}

#[test]
fn test_root_json_pointer_slash_is_empty() {
    let issues = inspect_json(&json!({"name": 123}), create_schema(), Some("/")).unwrap();
    assert_eq!(issues[0].json_pointer, "/name");
}

#[test]
fn test_ecma_lookahead_pattern() {
    let schema = to_map(json!({"pattern": "^(?![./~])(?!.*://)[^/]+$"}));
    assert_eq!(
        inspect_json(&json!("data.csv"), schema.clone(), None).unwrap(),
        vec![]
    );
    assert_eq!(
        inspect_json(&json!("../data.csv"), schema, None)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn test_format_is_not_asserted() {
    let schema = to_map(json!({"format": "email"}));
    assert_eq!(
        inspect_json(&json!("not-an-email"), schema, None).unwrap(),
        vec![]
    );
}
