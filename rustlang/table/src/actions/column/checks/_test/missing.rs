use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::missing::check_cell_missing;
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
fn test_non_nullable_column_with_nulls() {
    let column = create_column("id", json!({"type": "number"}));
    let table = df!(
        "source" => [Some(1.0), None, Some(3.0)],
        "target" => [Some(1.0), None, Some(3.0)],
    )
    .unwrap();

    let result = check_cell_missing(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap()["type"],
        "cell/missing"
    );
    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_nullable_column_returns_none() {
    let mut column = create_column("id", json!({"type": "number"}));
    column.set_nullable(Some(true));
    assert!(check_cell_missing(&column, &create_mapping()).is_none());
}

#[test]
fn test_unknown_column_returns_none() {
    let column = create_column("id", json!({}));
    assert!(check_cell_missing(&column, &create_mapping()).is_none());
}

#[test]
fn test_non_nullable_column_no_nulls() {
    let column = create_column("id", json!({"type": "number"}));
    let table = df!("source" => [1.0, 2.0, 3.0], "target" => [1.0, 2.0, 3.0]).unwrap();

    let result = check_cell_missing(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_error_template_fields() {
    let column = create_column("id", json!({"type": "number"}));

    let result = check_cell_missing(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap(),
        json!({"type": "cell/missing", "columnName": "id", "rowNumber": 0, "cell": ""})
    );
}
