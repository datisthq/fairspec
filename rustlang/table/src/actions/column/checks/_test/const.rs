use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::r#const::check_cell_const;
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
fn test_returns_none_when_no_const() {
    let column = create_column("status", json!({"type": "string"}));
    assert!(check_cell_const(&column, &create_mapping()).is_none());
}

#[test]
fn test_string_const_match() {
    let column = create_column("status", json!({"type": "string", "const": "active"}));
    let table = df!(
        "source" => ["active", "active", "active"],
        "target" => ["active", "active", "active"],
    )
    .unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["type"],
        "cell/const"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_string_const_mismatch() {
    let column = create_column("status", json!({"type": "string", "const": "active"}));
    let table = df!(
        "source" => ["active", "inactive", "active", "pending"],
        "target" => ["active", "inactive", "active", "pending"],
    )
    .unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_null_values_not_flagged() {
    let column = create_column("status", json!({"type": "string", "const": "active"}));
    let table = df!(
        "source" => [Some("active"), None, Some("active"), None],
        "target" => [Some("active"), None, Some("active"), None],
    )
    .unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_case_sensitivity() {
    let column = create_column("status", json!({"type": "string", "const": "active"}));
    let table = df!(
        "source" => ["Active", "ACTIVE", "active"],
        "target" => ["Active", "ACTIVE", "active"],
    )
    .unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_integer_const() {
    let column = create_column("priority", json!({"type": "integer", "const": 1}));
    let table = df!("source" => [1i64, 1, 2], "target" => [1i64, 1, 2]).unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["const"],
        "1"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_number_const() {
    let column = create_column("rate", json!({"type": "number", "const": 1.5}));
    let table = df!("source" => [1.5, 1.5, 2.5], "target" => [1.5, 1.5, 2.5]).unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["const"],
        "1.5"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_boolean_const() {
    let column = create_column("enabled", json!({"type": "boolean", "const": true}));
    let table = df!("source" => [true, true, false], "target" => [true, true, false]).unwrap();

    let result = check_cell_const(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["const"],
        "True"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}
