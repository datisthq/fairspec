use fairspec_metadata::Column;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::column::checks::min_items::check_cell_min_items;
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

fn create_table(sources: &[Option<&str>]) -> DataFrame {
    let target: ListChunked = sources
        .iter()
        .map(|source| {
            source.map(|text| Series::new("".into(), text.split(",").collect::<Vec<_>>()))
        })
        .collect();
    let source = Series::new("source".into(), sources);
    DataFrame::new_infer_height(vec![
        source.into_column(),
        target
            .into_series()
            .with_name("target".into())
            .into_column(),
    ])
    .unwrap()
}

#[test]
fn test_returns_none_for_non_list_column() {
    let column = create_column("tags", json!({"type": "string"}));
    assert!(check_cell_min_items(&column, &create_mapping()).is_none());
}

#[test]
fn test_returns_none_when_no_min_items() {
    let column = create_column("tags", json!({"type": "string", "format": "list"}));
    assert!(check_cell_min_items(&column, &create_mapping()).is_none());
}

#[test]
fn test_values_meeting_min_items() {
    let column = create_column(
        "tags",
        json!({"type": "string", "format": "list", "minItems": 3}),
    );
    let table = create_table(&[Some("a,b,c"), Some("x,y,z"), Some("1,2,3")]);

    let result = check_cell_min_items(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}

#[test]
fn test_values_below_min_items() {
    let column = create_column(
        "tags",
        json!({"type": "string", "format": "list", "minItems": 3}),
    );
    let table = create_table(&[Some("a,b,c"), Some("x"), Some("1,2"), Some("p,q,r,s")]);

    let result = check_cell_min_items(&column, &create_mapping()).unwrap();

    let template = serde_json::to_value(&result.error_template).unwrap();
    assert_eq!(template["type"], "cell/minItems");
    assert_eq!(template["minItems"], 3);
    assert_eq!(count_errors(table, result.is_error_expr), 2);
}

#[test]
fn test_null_values_not_flagged() {
    let column = create_column(
        "tags",
        json!({"type": "string", "format": "list", "minItems": 3}),
    );
    let table = create_table(&[Some("a,b,c"), None, Some("x,y,z")]);

    let result = check_cell_min_items(&column, &create_mapping()).unwrap();

    assert_eq!(count_errors(table, result.is_error_expr), 0);
}
