use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::min_length::check_cell_min_length;
use crate::models::cell::CellMapping;

fn create_column(name: &str, property: Value) -> Column {
    Column::from_property(name, serde_json::from_value(property).unwrap())
}

fn create_mapping() -> CellMapping {
    CellMapping {
        source: col("source"),
        target: col("target"),
    }
}

fn count_errors(table: DataFrame, is_error_expr: Expr) -> usize {
    table
        .lazy()
        .filter(is_error_expr)
        .collect()
        .unwrap()
        .height()
}

#[test]
fn test_returns_none_when_no_min_length() {
    let column = create_column("code", json!({"type": "string"}));
    assert!(check_cell_min_length(&column, &create_mapping()).is_none());
}

#[test]
fn test_returns_none_for_column_without_min_length_field() {
    let column = create_column("id", json!({"type": "integer"}));
    assert!(check_cell_min_length(&column, &create_mapping()).is_none());
}

#[test]
fn test_values_meeting_min_length() {
    let column = create_column("code", json!({"type": "string", "minLength": 3}));
    let table = df!(
        "source" => ["A123", "B456", "C789"],
        "target" => ["A123", "B456", "C789"],
    )
    .unwrap();

    let result = check_cell_min_length(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_below_min_length() {
    let column = create_column("username", json!({"type": "string", "minLength": 3}));
    let table = df!(
        "source" => ["bob", "a", "christopher", "ab"],
        "target" => ["bob", "a", "christopher", "ab"],
    )
    .unwrap();

    let result = check_cell_min_length(&column, &create_mapping()).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/minLength");
    assert_eq!(template["minLength"], 3);
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_null_values_not_flagged() {
    let column = create_column("code", json!({"type": "string", "minLength": 3}));
    let table = df!(
        "source" => [Some("ABCD"), Some("A"), None],
        "target" => [Some("ABCD"), Some("A"), None],
    )
    .unwrap();

    let result = check_cell_min_length(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 1);
}
