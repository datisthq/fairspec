use fairspec_metadata::TableSchema;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::table::normalize::normalize_table;
use crate::plugins::_shared::collect_records;

fn normalize(table: LazyFrame, properties: Value) -> Value {
    let schema: TableSchema = serde_json::from_value(json!({"properties": properties})).unwrap();
    collect_records(normalize_table(table, &schema).unwrap())
}

fn create_table() -> LazyFrame {
    df!("id" => [1i64, 2], "name" => ["english", "中文"])
        .unwrap()
        .lazy()
}

fn create_reversed_table() -> LazyFrame {
    df!("name" => ["english", "中文"], "id" => [1i64, 2])
        .unwrap()
        .lazy()
}

fn create_properties() -> Value {
    json!({"id": {"type": "integer"}, "name": {"type": "string"}})
}

fn create_records() -> Value {
    json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}])
}

#[test]
fn test_should_work_with_schema() {
    assert_eq!(
        normalize(create_table(), create_properties()),
        create_records()
    );
}

#[test]
fn test_should_work_with_less_fields_in_data() {
    let properties = json!({
        "id": {"type": "integer"},
        "name": {"type": "string"},
        "other": {"type": "boolean"},
    });
    assert_eq!(
        normalize(create_table(), properties),
        json!([
            {"id": 1, "name": "english", "other": null},
            {"id": 2, "name": "中文", "other": null},
        ])
    );
}

#[test]
fn test_should_work_with_more_fields_in_data() {
    let table = df!(
        "id" => [1i64, 2],
        "name" => ["english", "中文"],
        "other" => [true, false],
    )
    .unwrap()
    .lazy();
    assert_eq!(normalize(table, create_properties()), create_records());
}

#[test]
fn test_should_not_work_based_on_fields_order() {
    let table = df!("field1" => [1i64, 2], "field2" => ["english", "中文"])
        .unwrap()
        .lazy();
    assert_eq!(
        normalize(table, create_properties()),
        json!([{"id": null, "name": null}])
    );
}

#[test]
fn test_should_work_based_on_field_names_equal() {
    assert_eq!(
        normalize(create_reversed_table(), create_properties()),
        create_records()
    );
}

#[test]
fn test_should_work_based_on_field_names_subset() {
    assert_eq!(
        normalize(create_reversed_table(), create_properties()),
        create_records()
    );
}

#[test]
fn test_should_work_based_on_field_names_superset() {
    assert_eq!(
        normalize(create_reversed_table(), create_properties()),
        create_records()
    );
}

#[test]
fn test_should_work_based_on_field_names_partial() {
    assert_eq!(
        normalize(create_reversed_table(), create_properties()),
        create_records()
    );
}

#[test]
fn test_should_parse_string_columns() {
    let table = df!("id" => ["1", "2"], "name" => ["english", "中文"])
        .unwrap()
        .lazy();
    assert_eq!(normalize(table, create_properties()), create_records());
}

#[test]
fn test_should_read_type_errors_as_nulls() {
    let properties = json!({"id": {"type": "integer"}, "name": {"type": "integer"}});
    assert_eq!(
        normalize(create_table(), properties),
        json!([{"id": 1, "name": null}, {"id": 2, "name": null}])
    );
}
