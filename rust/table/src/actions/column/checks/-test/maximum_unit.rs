use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::maximum::check_cell_maximum;
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
fn test_returns_none_when_no_maximum() {
    let column = create_column("price", json!({"type": "number"}));
    assert!(check_cell_maximum(&column, &create_mapping(), false).is_none());
}

#[test]
fn test_returns_none_for_column_without_maximum_field() {
    let column = create_column("name", json!({"type": "string"}));
    assert!(check_cell_maximum(&column, &create_mapping(), false).is_none());
}

#[test]
fn test_values_within_maximum() {
    let column = create_column("price", json!({"type": "number", "maximum": 50}));
    let table = df!("source" => [10.5, 20.75, 30.0], "target" => [10.5, 20.75, 30.0]).unwrap();

    let result = check_cell_maximum(&column, &create_mapping(), false).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_exceeding_maximum() {
    let column = create_column("temperature", json!({"type": "number", "maximum": 40}));
    let table = df!(
        "source" => [20.5, 30.0, 40.0, 50.5],
        "target" => [20.5, 30.0, 40.0, 50.5],
    )
    .unwrap();

    let result = check_cell_maximum(&column, &create_mapping(), false).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/maximum");
    assert_eq!(template["maximum"], "40");
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_exclusive_maximum() {
    let column = create_column(
        "temperature",
        json!({"type": "number", "exclusiveMaximum": 40}),
    );
    let table = df!(
        "source" => [20.5, 30.0, 40.0, 50.5],
        "target" => [20.5, 30.0, 40.0, 50.5],
    )
    .unwrap();

    let result = check_cell_maximum(&column, &create_mapping(), true).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/exclusiveMaximum");
    assert_eq!(template["maximum"], "40");
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_integer_maximum() {
    let column = create_column("year", json!({"type": "integer", "maximum": 2022}));
    let table = df!("source" => [2020i64, 2021, 2023], "target" => [2020i64, 2021, 2023]).unwrap();

    let result = check_cell_maximum(&column, &create_mapping(), false).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["maximum"],
        "2022"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_integer_exclusive_maximum() {
    let column = create_column("year", json!({"type": "integer", "exclusiveMaximum": 2022}));
    let table = df!(
        "source" => [2020i64, 2021, 2022, 2023],
        "target" => [2020i64, 2021, 2022, 2023],
    )
    .unwrap();

    let result = check_cell_maximum(&column, &create_mapping(), true).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 2);
}
