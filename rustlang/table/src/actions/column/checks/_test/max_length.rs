use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::max_length::check_cell_max_length;
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
fn test_returns_none_when_no_max_length() {
    let column = create_column("code", json!({"type": "string"}));
    assert!(check_cell_max_length(&column, &create_mapping()).is_none());
}

#[test]
fn test_returns_none_for_column_without_max_length_field() {
    let column = create_column("id", json!({"type": "integer"}));
    assert!(check_cell_max_length(&column, &create_mapping()).is_none());
}

#[test]
fn test_values_within_max_length() {
    let column = create_column("code", json!({"type": "string", "maxLength": 4}));
    let table = df!(
        "source" => ["A123", "B456", "C789"],
        "target" => ["A123", "B456", "C789"],
    )
    .unwrap();

    let result = check_cell_max_length(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_exceeding_max_length() {
    let column = create_column("username", json!({"type": "string", "maxLength": 8}));
    let table = df!(
        "source" => ["bob", "alice", "christopher", "david"],
        "target" => ["bob", "alice", "christopher", "david"],
    )
    .unwrap();

    let result = check_cell_max_length(&column, &create_mapping()).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/maxLength");
    assert_eq!(template["maxLength"], 8);
    let errors = table.lazy().filter(result.is_error_expr).collect().unwrap();
    assert_eq!(errors.height(), 1);
    let sources = errors.column("source").unwrap().str().unwrap().clone();
    assert_eq!(sources.get(0), Some("christopher"));
}
