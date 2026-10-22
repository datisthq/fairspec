use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::r#type::check_cell_type;
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
fn test_no_errors_for_valid_values() {
    let column = create_column("id", json!({"type": "integer"}));
    let table = df!("source" => ["1", "2", "3", "4"], "target" => [1i64, 2, 3, 4]).unwrap();

    let result = check_cell_type(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_errors_for_invalid_integer_values() {
    let column = create_column("id", json!({"type": "integer"}));
    let table = df!(
        "source" => ["1", "bad", "3", "4x"],
        "target" => [Some(1i64), None, Some(3), None],
    )
    .unwrap();

    let result = check_cell_type(&column, &create_mapping()).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/type");
    assert_eq!(template["columnType"], "integer");
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_errors_for_invalid_number_values() {
    let column = create_column("price", json!({"type": "number"}));
    let table = df!(
        "source" => ["10.5", "twenty", "30.75", "$40"],
        "target" => [Some(10.5), None, Some(30.75), None],
    )
    .unwrap();

    let result = check_cell_type(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_errors_for_invalid_boolean_values() {
    let column = create_column("active", json!({"type": "boolean"}));
    let table = df!(
        "source" => ["true", "yes", "false", "0", "1"],
        "target" => [Some(true), None, Some(false), Some(false), Some(true)],
    )
    .unwrap();

    let result = check_cell_type(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 1);
}

#[test]
fn test_null_source_not_flagged() {
    let column = create_column("id", json!({"type": "integer"}));
    let table = df!(
        "source" => [Some("1"), None, Some("3")],
        "target" => [Some(1i64), None, Some(3)],
    )
    .unwrap();

    let result = check_cell_type(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_error_template_fields() {
    let column = create_column("name", json!({"type": "string"}));

    let result = check_cell_type(&column, &create_mapping()).unwrap();

    assert_eq!(
        serde_json::to_value(&result.error_template).unwrap(),
        json!({
            "type": "cell/type",
            "columnName": "name",
            "rowNumber": 0,
            "cell": "",
            "columnType": "string",
        })
    );
}
