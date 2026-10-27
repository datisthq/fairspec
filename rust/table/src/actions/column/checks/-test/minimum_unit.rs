use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::minimum::check_cell_minimum;
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
fn test_returns_none_when_no_minimum() {
    let column = create_column("price", json!({"type": "number"}));
    assert!(check_cell_minimum(&column, &create_mapping(), false).is_none());
}

#[test]
fn test_returns_none_for_column_without_minimum_field() {
    let column = create_column("name", json!({"type": "string"}));
    assert!(check_cell_minimum(&column, &create_mapping(), false).is_none());
}

#[test]
fn test_values_above_minimum() {
    let column = create_column("price", json!({"type": "number", "minimum": 5}));
    let table = df!("source" => [10.5, 20.75, 30.0], "target" => [10.5, 20.75, 30.0]).unwrap();

    let result = check_cell_minimum(&column, &create_mapping(), false).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_below_minimum() {
    let column = create_column("temperature", json!({"type": "number", "minimum": 10}));
    let table = df!(
        "source" => [20.5, 30.0, 40.0, 3.5],
        "target" => [20.5, 30.0, 40.0, 3.5],
    )
    .unwrap();

    let result = check_cell_minimum(&column, &create_mapping(), false).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/minimum");
    assert_eq!(template["minimum"], "10");
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_fractional_minimum_renders_as_is() {
    let column = create_column("temperature", json!({"type": "number", "minimum": 1.5}));

    let result = check_cell_minimum(&column, &create_mapping(), false).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["minimum"],
        "1.5"
    );
}

#[test]
fn test_exclusive_minimum() {
    let column = create_column(
        "temperature",
        json!({"type": "number", "exclusiveMinimum": 10}),
    );
    let table = df!(
        "source" => [20.5, 30.0, 10.0, 5.5],
        "target" => [20.5, 30.0, 10.0, 5.5],
    )
    .unwrap();

    let result = check_cell_minimum(&column, &create_mapping(), true).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/exclusiveMinimum");
    assert_eq!(template["minimum"], "10");
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_integer_minimum() {
    let column = create_column("year", json!({"type": "integer", "minimum": 2019}));
    let table = df!("source" => [2020i64, 2021, 2018], "target" => [2020i64, 2021, 2018]).unwrap();

    let result = check_cell_minimum(&column, &create_mapping(), false).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["minimum"],
        "2019"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_integer_exclusive_minimum() {
    let column = create_column("year", json!({"type": "integer", "exclusiveMinimum": 2019}));
    let table = df!(
        "source" => [2020i64, 2021, 2019, 2018],
        "target" => [2020i64, 2021, 2019, 2018],
    )
    .unwrap();

    let result = check_cell_minimum(&column, &create_mapping(), true).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_integer_categorical_reads_the_cell_value() {
    let column = create_column(
        "severity",
        json!({"type": "integer", "format": "categorical", "categories": [1, 2, 3], "minimum": 2}),
    );
    let table = df!("source" => ["1", "2", "3.0"], "target" => ["1", "2", "3"]).unwrap();

    let result = check_cell_minimum(&column, &create_mapping(), false).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 1);
}
