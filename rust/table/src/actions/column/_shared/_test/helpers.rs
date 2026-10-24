use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::_shared::helpers::{inspect_json_column, inspect_text_column};

fn create_column(name: &str, property: Value, nullable: bool) -> Column {
    let mut column = Column::from_property(name, serde_json::from_value(property).unwrap());
    if nullable {
        column.set_nullable(Some(true));
    }
    column
}

fn create_duration_column(nullable: bool) -> Column {
    create_column(
        "duration",
        json!({"type": "string", "format": "duration"}),
        nullable,
    )
}

fn parse_period(text: &str) -> bool {
    text.starts_with('P')
}

fn inspect_text(
    column: &Column,
    cells: &[Option<&str>],
    parse: &dyn Fn(&str) -> bool,
) -> Vec<Value> {
    let table = df!(column.name() => cells).unwrap().lazy();
    let errors = inspect_text_column(column, table, parse, None).unwrap();
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

fn inspect_json(
    column: &Column,
    cells: &[Option<&str>],
    type_json_schema: Option<&Value>,
) -> Vec<Value> {
    let table = df!(column.name() => cells).unwrap().lazy();
    let errors = inspect_json_column(column, table, type_json_schema, None).unwrap();
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

#[test]
fn test_no_errors_for_valid_values() {
    let cells = [Some("P1Y"), Some("P2M"), Some("P3D")];
    assert!(inspect_text(&create_duration_column(false), &cells, &parse_period).is_empty());
}

#[test]
fn test_error_for_invalid_values() {
    let cells = [Some("P1Y"), Some("invalid"), Some("P3D")];

    let errors = inspect_text(&create_duration_column(false), &cells, &parse_period);

    assert_eq!(
        errors,
        vec![json!({
            "type": "cell/type",
            "columnName": "duration",
            "rowNumber": 2,
            "cell": "invalid",
            "columnType": "duration",
        })]
    );
}

#[test]
fn test_skip_null_values() {
    let cells = [Some("P1Y"), None, Some("P3D")];
    assert!(inspect_text(&create_duration_column(true), &cells, &parse_period).is_empty());
}

#[test]
fn test_error_for_missing_values_in_non_null_column() {
    let column = create_column(
        "duration",
        json!({"type": "string", "format": "duration", "missingValues": ["NA"]}),
        false,
    );
    let cells = [Some("P1Y"), None, Some("NA")];

    let errors = inspect_text(&column, &cells, &parse_period);

    assert_eq!(
        errors,
        vec![
            json!({"type": "cell/missing", "columnName": "duration", "rowNumber": 2, "cell": ""}),
            json!({"type": "cell/missing", "columnName": "duration", "rowNumber": 3, "cell": "NA"}),
        ]
    );
}

#[test]
fn test_error_when_parse_raises() {
    let errors = inspect_text(&create_duration_column(false), &[Some("bad")], &|_| false);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
}

fn create_object_column(nullable: bool) -> Column {
    create_column("data", json!({"type": "object"}), nullable)
}

fn create_array_column() -> Column {
    create_column("data", json!({"type": "array"}), false)
}

fn create_name_json_schema() -> Value {
    json!({
        "type": "object",
        "properties": {"name": {"type": "string"}},
        "required": ["name"],
    })
}

#[test]
fn test_no_errors_for_valid_object() {
    assert!(inspect_json(&create_object_column(false), &[Some(r#"{"a": 1}"#)], None).is_empty());
}

#[test]
fn test_error_for_invalid_json() {
    let errors = inspect_json(&create_object_column(false), &[Some("not json")], None);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
    assert_eq!(errors[0]["cell"], "not json");
}

#[test]
fn test_error_for_array_when_expecting_object() {
    let errors = inspect_json(&create_object_column(false), &[Some("[1, 2, 3]")], None);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
}

#[test]
fn test_no_errors_for_valid_array() {
    assert!(inspect_json(&create_array_column(), &[Some("[1, 2, 3]")], None).is_empty());
}

#[test]
fn test_error_for_object_when_expecting_array() {
    let errors = inspect_json(&create_array_column(), &[Some(r#"{"a": 1}"#)], None);

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
}

#[test]
fn test_skip_null_values_in_json_column() {
    let cells = [None, Some(r#"{"a": 1}"#)];
    assert!(inspect_json(&create_object_column(true), &cells, None).is_empty());
}

#[test]
fn test_error_for_null_values_in_non_null_column() {
    let cells = [None, Some(r#"{"a": 1}"#)];

    let errors = inspect_json(&create_object_column(false), &cells, None);

    assert_eq!(
        errors,
        vec![json!({"type": "cell/missing", "columnName": "data", "rowNumber": 1, "cell": ""})]
    );
}

#[test]
fn test_type_json_schema_valid() {
    let schema = create_name_json_schema();
    let cells = [Some(r#"{"name": "test"}"#)];
    assert!(inspect_json(&create_object_column(false), &cells, Some(&schema)).is_empty());
}

#[test]
fn test_type_json_schema_invalid() {
    let schema = create_name_json_schema();

    let errors = inspect_json(
        &create_object_column(false),
        &[Some(r#"{"name": 123}"#)],
        Some(&schema),
    );

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["type"], "cell/type");
}
