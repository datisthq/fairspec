use fairspec_metadata::{Column, TableSchema};
use serde_json::{Value, json};

use crate::actions::table::_shared::helpers::merge_missing_values;

fn create_column(property: Value) -> Column {
    serde_json::from_value(json!({"name": "name", "type": "string", "property": property})).unwrap()
}

fn create_schema(value: Value) -> TableSchema {
    serde_json::from_value(value).unwrap()
}

fn get_missing_values(column: &Column) -> Value {
    serde_json::to_value(column.property()).unwrap()["missingValues"].clone()
}

#[test]
fn test_no_missing_values_in_schema() {
    let column = create_column(json!({"type": "string"}));
    let result = merge_missing_values(&column, &create_schema(json!({})));
    assert_eq!(result, column);
}

#[test]
fn test_merge_string_missing_values() {
    let column = create_column(json!({"type": "string"}));
    let schema = create_schema(json!({"missingValues": ["", "NA", "N/A"]}));
    let result = merge_missing_values(&column, &schema);
    assert_eq!(get_missing_values(&result), json!(["", "NA", "N/A"]));
    assert_ne!(result, column);
}

#[test]
fn test_merge_with_existing_column_missing_values() {
    let column = create_column(json!({"type": "string", "missingValues": ["custom"]}));
    let schema = create_schema(json!({"missingValues": ["", "NA"]}));
    let result = merge_missing_values(&column, &schema);
    assert_eq!(get_missing_values(&result), json!(["custom", "", "NA"]));
}

#[test]
fn test_does_not_mutate_original_column() {
    let column = create_column(json!({"type": "string", "missingValues": ["original"]}));
    let schema = create_schema(json!({"missingValues": ["added"]}));
    merge_missing_values(&column, &schema);
    assert_eq!(get_missing_values(&column), json!(["original"]));
}

#[test]
fn test_skip_integer_missing_values_for_string_columns() {
    let column = create_column(json!({"type": "string"}));
    let schema = create_schema(json!({"missingValues": ["", 99]}));
    let result = merge_missing_values(&column, &schema);
    assert_eq!(get_missing_values(&result), json!([""]));
}

#[test]
fn test_skip_integer_missing_value_item_for_string_columns() {
    let column = create_column(json!({"type": "string"}));
    let schema = create_schema(json!({"missingValues": ["", {"value": 99, "label": "missing"}]}));
    let result = merge_missing_values(&column, &schema);
    assert_eq!(get_missing_values(&result), json!([""]));
}
