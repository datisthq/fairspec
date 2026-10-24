use fairspec_metadata::ArrayColumn;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::types::array::inspect_array_column;

fn create_column(name: &str, property: Value) -> ArrayColumn {
    serde_json::from_value(json!({"name": name, "property": property})).unwrap()
}

fn inspect(column: &ArrayColumn, name: &str, cells: &[Option<&str>]) -> Vec<Value> {
    let table = df!(name => cells).unwrap().lazy();
    let errors = inspect_array_column(column, table, None).unwrap();
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

#[test]
fn test_valid_json_arrays() {
    let column = create_column("tags", json!({"type": "array"}));
    let cells = [
        Some(r#"["tag1","tag2"]"#),
        Some("[1,2,3]"),
        Some(r#"["a","b","c"]"#),
    ];
    assert!(inspect(&column, "tags", &cells).is_empty());
}

#[test]
fn test_empty_arrays() {
    let column = create_column("items", json!({"type": "array"}));
    assert!(inspect(&column, "items", &[Some("[]"), Some("[]"), Some("[]")]).is_empty());
}

#[test]
fn test_null_values() {
    let mut column = create_column("data", json!({"type": "array"}));
    column.nullable = Some(true);
    let cells = [Some(r#"["value"]"#), None, Some("[1,2,3]")];
    assert!(inspect(&column, "data", &cells).is_empty());
}

#[test]
fn test_json_objects_error() {
    let column = create_column("data", json!({"type": "array"}));
    let cells = [
        Some("[1,2,3]"),
        Some(r#"{"key":"value"}"#),
        Some(r#"["a","b"]"#),
    ];

    let errors = inspect(&column, "data", &cells);

    assert_eq!(
        errors,
        vec![json!({
            "type": "cell/type",
            "columnName": "data",
            "rowNumber": 2,
            "cell": r#"{"key":"value"}"#,
            "columnType": "array",
        })]
    );
}

#[test]
fn test_invalid_json_error() {
    let column = create_column("data", json!({"type": "array"}));
    let cells = [
        Some(r#"["valid"]"#),
        Some("invalid json"),
        Some("[1,2,3]"),
        Some("[broken"),
    ];

    let errors = inspect(&column, "data", &cells);

    let type_errors: Vec<(Value, Value)> = errors
        .iter()
        .filter(|error| error["type"] == "cell/type")
        .map(|error| (error["rowNumber"].clone(), error["cell"].clone()))
        .collect();
    assert_eq!(
        type_errors,
        vec![
            (json!(2), json!("invalid json")),
            (json!(4), json!("[broken")),
        ]
    );
}

#[test]
fn test_nested_arrays() {
    let column = create_column("matrix", json!({"type": "array"}));
    let cells = [
        Some("[[1,2],[3,4]]"),
        Some("[[5,6],[7,8]]"),
        Some(r#"[["a","b"],["c","d"]]"#),
    ];
    assert!(inspect(&column, "matrix", &cells).is_empty());
}

#[test]
fn test_empty_strings_error() {
    let column = create_column("data", json!({"type": "array"}));
    let cells = [Some(r#"["valid"]"#), Some(""), Some("[1,2,3]")];

    let errors = inspect(&column, "data", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(errors[0]["rowNumber"], 2);
    assert_eq!(errors[0]["cell"], "");
}

#[test]
fn test_json_primitives_error() {
    let column = create_column("data", json!({"type": "array"}));
    let cells = [
        Some(r#""string""#),
        Some("123"),
        Some("true"),
        Some("false"),
        Some("null"),
    ];

    let errors = inspect(&column, "data", &cells);

    let located: Vec<(Value, Value)> = errors
        .iter()
        .map(|error| (error["rowNumber"].clone(), error["cell"].clone()))
        .collect();
    assert_eq!(
        located,
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
fn test_valid_arrays_matching_json_schema() {
    let column = create_column(
        "scores",
        json!({"type": "array", "items": {"type": "number"}, "minItems": 3, "maxItems": 3}),
    );
    let cells = [Some("[80,90,100]"), Some("[75,85,95]"), Some("[90,95,100]")];
    assert!(inspect(&column, "scores", &cells).is_empty());
}

#[test]
fn test_arrays_not_matching_json_schema() {
    let column = create_column(
        "numbers",
        json!({"type": "array", "items": {"type": "number"}, "minItems": 2}),
    );
    let cells = [
        Some("[1,2,3]"),
        Some(r#"["not","numbers"]"#),
        Some("[1]"),
        Some("[4,5,6]"),
    ];

    let errors = inspect(&column, "numbers", &cells);

    assert_eq!(
        errors,
        vec![
            json!({
                "type": "cell/json",
                "columnName": "numbers",
                "rowNumber": 2,
                "cell": r#"["not","numbers"]"#,
                "message": r#""not" is not of type "number""#,
                "jsonPointer": "/0",
            }),
            json!({
                "type": "cell/json",
                "columnName": "numbers",
                "rowNumber": 2,
                "cell": r#"["not","numbers"]"#,
                "message": r#""numbers" is not of type "number""#,
                "jsonPointer": "/1",
            }),
            json!({
                "type": "cell/json",
                "columnName": "numbers",
                "rowNumber": 3,
                "cell": "[1]",
                "message": "[1] has less than 2 items",
                "jsonPointer": "",
            }),
        ]
    );
}

#[test]
fn test_complex_json_schema_with_array_of_objects() {
    let column = create_column(
        "users",
        json!({
            "type": "array",
            "items": {
                "type": "object",
                "properties": {"name": {"type": "string"}, "age": {"type": "number"}},
                "required": ["name", "age"],
            },
        }),
    );
    let cells = [
        Some(r#"[{"name":"John","age":30},{"name":"Jane","age":25}]"#),
        Some(r#"[{"name":"Bob","age":"invalid"}]"#),
    ];

    let errors = inspect(&column, "users", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/json");
    assert_eq!(errors[0]["rowNumber"], 2);
    assert_eq!(errors[0]["cell"], r#"[{"name":"Bob","age":"invalid"}]"#);
    assert_eq!(errors[0]["message"], r#""invalid" is not of type "number""#);
    assert_eq!(errors[0]["jsonPointer"], "/0/age");
}

#[test]
fn test_json_schema_with_unique_items() {
    let column = create_column(
        "tags",
        json!({"type": "array", "items": {"type": "string"}, "uniqueItems": true}),
    );
    let cells = [
        Some(r#"["unique","values"]"#),
        Some(r#"["duplicate","duplicate"]"#),
    ];

    let errors = inspect(&column, "tags", &cells);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/json");
    assert_eq!(errors[0]["rowNumber"], 2);
    assert_eq!(errors[0]["cell"], r#"["duplicate","duplicate"]"#);
    assert!(
        errors[0]["message"]
            .as_str()
            .unwrap()
            .contains("has non-unique elements")
    );
    assert_eq!(errors[0]["jsonPointer"], "");
}
