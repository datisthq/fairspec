use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::pattern::check_cell_pattern;
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

const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

#[test]
fn test_returns_none_when_no_pattern() {
    let column = create_column("email", json!({"type": "string"}));
    assert!(check_cell_pattern(&column, &create_mapping()).is_none());
}

#[test]
fn test_returns_none_for_column_without_pattern_field() {
    let column = create_column("id", json!({"type": "integer"}));
    assert!(check_cell_pattern(&column, &create_mapping()).is_none());
}

#[test]
fn test_values_matching_pattern() {
    let column = create_column("email", json!({"type": "string", "pattern": EMAIL_PATTERN}));
    let table = df!(
        "source" => ["john@example.com", "alice@domain.org", "test@test.io"],
        "target" => ["john@example.com", "alice@domain.org", "test@test.io"],
    )
    .unwrap();

    let result = check_cell_pattern(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_not_matching_pattern() {
    let column = create_column("email", json!({"type": "string", "pattern": EMAIL_PATTERN}));
    let table = df!(
        "source" => ["john@example.com", "alice@domain", "test.io", "valid@email.com"],
        "target" => ["john@example.com", "alice@domain", "test.io", "valid@email.com"],
    )
    .unwrap();

    let result = check_cell_pattern(&column, &create_mapping()).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/pattern");
    assert_eq!(template["pattern"], EMAIL_PATTERN);
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}
