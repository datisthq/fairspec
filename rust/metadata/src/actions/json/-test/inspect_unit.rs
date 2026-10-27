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
    inspect_json(&value, create_schema(), None, None).unwrap()
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
    let issues = inspect_json(&json!({"a/b": "x", "c~d": "x"}), schema, None, None).unwrap();
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
    let issues = inspect_json(&json!({"name": 123}), create_schema(), Some("/root"), None).unwrap();
    assert_eq!(issues[0].json_pointer, "/root/name");
}

#[test]
fn test_root_json_pointer_slash_is_empty() {
    let issues = inspect_json(&json!({"name": 123}), create_schema(), Some("/"), None).unwrap();
    assert_eq!(issues[0].json_pointer, "/name");
}

#[test]
fn test_ecma_lookahead_pattern() {
    let schema = to_map(json!({"pattern": "^(?![./~])(?!.*://)[^/]+$"}));
    assert_eq!(
        inspect_json(&json!("data.csv"), schema.clone(), None, None).unwrap(),
        vec![]
    );
    assert_eq!(
        inspect_json(&json!("../data.csv"), schema, None, None)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn test_format_is_not_asserted() {
    let schema = to_map(json!({"format": "email"}));
    assert_eq!(
        inspect_json(&json!("not-an-email"), schema, None, None).unwrap(),
        vec![]
    );
}

#[test]
fn test_const_object_ignores_key_order() {
    let schema = to_map(json!({"const": {"x": 1, "y": [1, 2]}}));
    let issues = inspect_json(&json!({"y": [1, 2], "x": 1}), schema, None, None).unwrap();
    assert_eq!(issues, vec![]);
}

#[test]
fn test_unique_items_compares_objects_by_members() {
    let schema = to_map(json!({"uniqueItems": true}));
    let issues = inspect_json(
        &json!([{"x": 1, "y": 2}, {"y": 2, "x": 1}]),
        schema,
        None,
        None,
    )
    .unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].json_pointer, "");
}

#[test]
fn test_unique_items_compares_objects_by_members_in_long_arrays() {
    let schema = to_map(json!({"uniqueItems": true}));
    let mut items: Vec<Value> = (0..20).map(|n| json!({"x": n, "y": n})).collect();
    items.push(json!({"y": 5, "x": 5}));
    let issues = inspect_json(&Value::Array(items), schema, None, None).unwrap();
    assert_eq!(issues.len(), 1);
}

#[test]
fn test_reports_an_if_then_failure_once_at_the_failing_then() {
    let schema = to_map(json!({
        "if": {"properties": {"kind": {"const": "a"}}, "required": ["kind"]},
        "then": {"properties": {"value": {"type": "string"}}},
    }));
    let issues = inspect_json(&json!({"kind": "a", "value": 1}), schema, None, None).unwrap();
    let pointers: Vec<_> = issues.into_iter().map(|issue| issue.json_pointer).collect();
    assert_eq!(pointers, vec!["/value"]);
}

#[test]
fn test_asserts_only_the_given_formats() {
    static FORMATS: [crate::actions::json::inspect::FormatCheck; 1] =
        [("even", |value| value.len() % 2 == 0)];
    let schema = to_map(json!({
        "properties": {
            "a": {"type": "string", "format": "even"},
            "b": {"type": "string", "format": "uri"},
        },
    }));
    let issues =
        inspect_json(&json!({"a": "odd", "b": "x"}), schema, None, Some(&FORMATS)).unwrap();
    let pointers: Vec<_> = issues.into_iter().map(|issue| issue.json_pointer).collect();
    assert_eq!(pointers, vec!["/a"]);
}
