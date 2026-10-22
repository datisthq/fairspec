use fairspec_metadata::ObjectColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::object::inspect_object_column;

fn create_column(name: &str, property: Value) -> ObjectColumn {
    serde_json::from_value(json!({"name": name, "property": property})).unwrap()
}

fn inspect(column: &ObjectColumn, name: &str, cells: &[Option<&str>]) -> Vec<Value> {
    let table = df!(name => cells).unwrap().lazy();
    let errors = inspect_object_column(column, table, None).unwrap();
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

fn locate(errors: &[Value]) -> Vec<(Value, Value)> {
    errors
        .iter()
        .map(|error| (error["rowNumber"].clone(), error["cell"].clone()))
        .collect()
}

fn create_user_property() -> Value {
    json!({
        "type": "object",
        "properties": {"name": {"type": "string"}, "age": {"type": "number"}},
        "required": ["name", "age"],
    })
}

#[test]
fn test_valid_json_objects() {
    let column = create_column("metadata", json!({"type": "object"}));
    let cells = [
        Some(r#"{"key":"value"}"#),
        Some(r#"{"num":123}"#),
        Some(r#"{"arr":[1,2,3]}"#),
    ];
    assert!(inspect(&column, "metadata", &cells).is_empty());
}

#[test]
fn test_json_arrays_error() {
    let column = create_column("data", json!({"type": "object"}));
    let cells = [
        Some("[1,2,3]"),
        Some(r#"{"key":"value"}"#),
        Some(r#"["a","b","c"]"#),
    ];

    let errors = inspect(&column, "data", &cells);

    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(errors[0]["columnName"], "data");
    assert_eq!(errors[0]["columnType"], "object");
    assert_eq!(
        locate(&errors),
        vec![
            (json!(1), json!("[1,2,3]")),
            (json!(3), json!(r#"["a","b","c"]"#))
        ]
    );
}

#[test]
fn test_null_values() {
    let mut column = create_column("config", json!({"type": "object"}));
    column.nullable = Some(true);
    let cells = [Some(r#"{"key":"value"}"#), None, Some(r#"{"num":123}"#)];
    assert!(inspect(&column, "config", &cells).is_empty());
}

#[test]
fn test_invalid_json_error() {
    let column = create_column("data", json!({"type": "object"}));
    let cells = [
        Some(r#"{"valid":true}"#),
        Some("invalid json"),
        Some(r#"{"key":"value"}"#),
        Some("{broken}"),
    ];

    let errors = inspect(&column, "data", &cells);

    assert!(errors.iter().all(|error| error["type"] == "cell/type"));
    assert_eq!(
        locate(&errors),
        vec![
            (json!(2), json!("invalid json")),
            (json!(4), json!("{broken}"))
        ]
    );
}

#[test]
fn test_complex_nested_json() {
    let column = create_column("complex", json!({"type": "object"}));
    let cells = [
        Some(r#"{"user":{"name":"John","age":30,"tags":["admin","user"]}}"#),
        Some(r#"{"nested":{"deep":{"value":true}}}"#),
        Some(r#"{"array":[{"id":1},{"id":2}]}"#),
    ];
    assert!(inspect(&column, "complex", &cells).is_empty());
}

#[test]
fn test_empty_strings_error() {
    let column = create_column("data", json!({"type": "object"}));
    let cells = [
        Some(r#"{"valid":true}"#),
        Some(""),
        Some(r#"{"key":"value"}"#),
    ];

    let errors = inspect(&column, "data", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(locate(&errors), vec![(json!(2), json!(""))]);
}

#[test]
fn test_json_primitives_error() {
    let column = create_column("data", json!({"type": "object"}));
    let cells = [
        Some(r#""string""#),
        Some("123"),
        Some("true"),
        Some("false"),
        Some("null"),
    ];

    let errors = inspect(&column, "data", &cells);

    assert_eq!(
        locate(&errors),
        vec![
            (json!(1), json!(r#""string""#)),
            (json!(2), json!("123")),
            (json!(3), json!("true")),
            (json!(4), json!("false")),
            (json!(5), json!("null")),
        ]
    );
}

#[test]
fn test_valid_objects_matching_json_schema() {
    let column = create_column("user", create_user_property());
    let cells = [
        Some(r#"{"name":"John","age":30}"#),
        Some(r#"{"name":"Jane","age":25}"#),
        Some(r#"{"name":"Bob","age":35}"#),
    ];
    assert!(inspect(&column, "user", &cells).is_empty());
}

#[test]
fn test_objects_not_matching_json_schema() {
    let column = create_column("user", create_user_property());
    let cells = [
        Some(r#"{"name":"John","age":30}"#),
        Some(r#"{"name":"Jane"}"#),
        Some(r#"{"age":25}"#),
        Some(r#"{"name":"Bob","age":"invalid"}"#),
    ];

    let errors = inspect(&column, "user", &cells);

    assert_eq!(
        errors,
        vec![
            json!({
                "type": "cell/json",
                "columnName": "user",
                "rowNumber": 2,
                "cell": r#"{"name":"Jane"}"#,
                "message": r#""age" is a required property"#,
                "jsonPointer": "",
            }),
            json!({
                "type": "cell/json",
                "columnName": "user",
                "rowNumber": 3,
                "cell": r#"{"age":25}"#,
                "message": r#""name" is a required property"#,
                "jsonPointer": "",
            }),
            json!({
                "type": "cell/json",
                "columnName": "user",
                "rowNumber": 4,
                "cell": r#"{"name":"Bob","age":"invalid"}"#,
                "message": r#""invalid" is not of type "number""#,
                "jsonPointer": "/age",
            }),
        ]
    );
}

#[test]
fn test_complex_json_schema_with_nested_objects() {
    let column = create_column(
        "config",
        json!({
            "type": "object",
            "properties": {
                "database": {
                    "type": "object",
                    "properties": {"host": {"type": "string"}, "port": {"type": "number"}},
                    "required": ["host", "port"],
                },
                "cache": {
                    "type": "object",
                    "properties": {"enabled": {"type": "boolean"}},
                    "required": ["enabled"],
                },
            },
            "required": ["database", "cache"],
        }),
    );
    let cells = [
        Some(r#"{"database":{"host":"localhost","port":5432},"cache":{"enabled":true}}"#),
        Some(r#"{"database":{"host":"localhost","port":"invalid"},"cache":{"enabled":true}}"#),
    ];

    let errors = inspect(&column, "config", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/json");
    assert_eq!(errors[0]["rowNumber"], 2);
    assert_eq!(errors[0]["message"], r#""invalid" is not of type "number""#);
    assert_eq!(errors[0]["jsonPointer"], "/database/port");
}

#[test]
fn test_json_schema_with_array_properties() {
    let column = create_column(
        "data",
        json!({
            "type": "object",
            "properties": {
                "items": {"type": "array", "items": {"type": "number"}},
                "name": {"type": "string"},
            },
            "required": ["items", "name"],
        }),
    );
    let cells = [
        Some(r#"{"items":[1,2,3],"name":"test"}"#),
        Some(r#"{"items":["not","numbers"],"name":"test"}"#),
    ];

    let errors = inspect(&column, "data", &cells);

    let issues: Vec<(Value, Value, Value)> = errors
        .iter()
        .map(|error| {
            (
                error["rowNumber"].clone(),
                error["message"].clone(),
                error["jsonPointer"].clone(),
            )
        })
        .collect();
    assert_eq!(
        issues,
        vec![
            (
                json!(2),
                json!(r#""not" is not of type "number""#),
                json!("/items/0")
            ),
            (
                json!(2),
                json!(r#""numbers" is not of type "number""#),
                json!("/items/1")
            ),
        ]
    );
}
