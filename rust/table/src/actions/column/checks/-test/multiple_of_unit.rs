use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::multiple_of::check_cell_multiple_of;
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
fn test_returns_none_when_no_multiple_of() {
    let column = create_column("quantity", json!({"type": "integer"}));
    assert!(check_cell_multiple_of(&column, &create_mapping()).is_none());
}

#[test]
fn test_decimal_multiples_are_exact() {
    let column = create_column("price", json!({"type": "number", "multipleOf": 0.01}));
    let table = df!("source" => [99.99, 0.07, 1.005], "target" => [99.99, 0.07, 1.005]).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    let errors = table.lazy().filter(result.is_error_expr).collect().unwrap();
    let targets: Vec<Option<f64>> = errors.column("target").unwrap().f64().unwrap().to_vec();
    assert_eq!(targets, vec![Some(1.005)]);
}

#[test]
fn test_returns_none_for_column_without_multiple_of_field() {
    let column = create_column("name", json!({"type": "string"}));
    assert!(check_cell_multiple_of(&column, &create_mapping()).is_none());
}

#[test]
fn test_valid_integer_multiples() {
    let column = create_column("quantity", json!({"type": "integer", "multipleOf": 10}));
    let table = df!("source" => [10i64, 20, 30, 40], "target" => [10i64, 20, 30, 40]).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_invalid_integer_values() {
    let column = create_column("quantity", json!({"type": "integer", "multipleOf": 10}));
    let table = df!("source" => [10i64, 15, 20], "target" => [10i64, 15, 20]).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/multipleOf");
    assert_eq!(template["multipleOf"], json!(10));
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_valid_number_multiples() {
    let column = create_column("price", json!({"type": "number", "multipleOf": 2.5}));
    let table = df!("source" => [2.5, 5.0, 7.5], "target" => [2.5, 5.0, 7.5]).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_invalid_number_values() {
    let column = create_column("price", json!({"type": "number", "multipleOf": 2.5}));
    let table = df!("source" => [2.5, 3.7, 5.0], "target" => [2.5, 3.7, 5.0]).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_multiple_of_one() {
    let column = create_column("count", json!({"type": "integer", "multipleOf": 1}));
    let table = df!("source" => [1i64, 2, 3, 4], "target" => [1i64, 2, 3, 4]).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_digits_past_double_precision() {
    let column = create_column("price", json!({"type": "number", "multipleOf": 0.1}));
    let source = ["0.3", "0.30000000000000001", "123456789012345678901.5"];
    let target: Vec<f64> = source.iter().map(|text| text.parse().unwrap()).collect();
    let table = df!("source" => source, "target" => target).unwrap();

    let result = check_cell_multiple_of(&column, &create_mapping()).unwrap();

    let errors = table.lazy().filter(result.is_error_expr).collect().unwrap();
    let sources: Vec<Option<&str>> = errors
        .column("source")
        .unwrap()
        .str()
        .unwrap()
        .iter()
        .collect();
    assert_eq!(sources, vec![Some("0.30000000000000001")]);
}
