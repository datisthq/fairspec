use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::r#enum::check_cell_enum;
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

fn create_status_column() -> Column {
    create_column(
        "status",
        json!({"type": "string", "enum": ["pending", "approved", "rejected"]}),
    )
}

#[test]
fn test_returns_none_when_no_enum() {
    let column = create_column("status", json!({"type": "string"}));
    assert!(check_cell_enum(&column, &create_mapping()).is_none());
}

#[test]
fn test_string_values_in_enum() {
    let table = df!(
        "source" => ["pending", "approved", "rejected", "pending"],
        "target" => ["pending", "approved", "rejected", "pending"],
    )
    .unwrap();

    let result = check_cell_enum(&create_status_column(), &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_not_in_enum() {
    let table = df!(
        "source" => ["pending", "approved", "unknown", "cancelled", "rejected"],
        "target" => ["pending", "approved", "unknown", "cancelled", "rejected"],
    )
    .unwrap();

    let result = check_cell_enum(&create_status_column(), &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["type"],
        "cell/enum"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_null_values_not_flagged() {
    let table = df!(
        "source" => [Some("pending"), None, Some("approved"), None],
        "target" => [Some("pending"), None, Some("approved"), None],
    )
    .unwrap();

    let result = check_cell_enum(&create_status_column(), &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_case_sensitivity() {
    let table = df!(
        "source" => ["Pending", "APPROVED", "rejected"],
        "target" => ["Pending", "APPROVED", "rejected"],
    )
    .unwrap();

    let result = check_cell_enum(&create_status_column(), &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_integer_enum() {
    let column = create_column("priority", json!({"type": "integer", "enum": [1, 2, 3]}));
    let table = df!("source" => [1i64, 2, 5], "target" => [1i64, 2, 5]).unwrap();

    let result = check_cell_enum(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["enum"],
        json!(["1", "2", "3"])
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_number_enum() {
    let column = create_column("rating", json!({"type": "number", "enum": [1.5, 2.5, 3.5]}));
    let table = df!("source" => [1.5, 2.5, 4.5], "target" => [1.5, 2.5, 4.5]).unwrap();

    let result = check_cell_enum(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["enum"],
        json!(["1.5", "2.5", "3.5"])
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}
