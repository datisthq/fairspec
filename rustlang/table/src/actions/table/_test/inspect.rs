use fairspec_metadata::TableSchema;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::table::inspect::inspect_table;

fn create_schema(value: Value) -> TableSchema {
    serde_json::from_value(value).unwrap()
}

fn inspect(
    table: LazyFrame,
    schema: &TableSchema,
    max_errors: Option<usize>,
    concurrency: Option<usize>,
) -> Vec<Value> {
    inspect_table(table, Some(schema), None, None, max_errors, concurrency)
        .unwrap()
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

fn missing(column_name: &str) -> Value {
    json!({"type": "column/missing", "columnName": column_name})
}

fn create_id_name_schema(extra: Value) -> TableSchema {
    let mut schema = json!({
        "properties": {"id": {"type": "number"}, "name": {"type": "string"}},
    });
    for (key, value) in extra.as_object().unwrap() {
        schema[key] = value.clone();
    }
    create_schema(schema)
}

#[test]
fn test_should_pass_when_columns_exactly_match() {
    let table = df!("id" => [1i64, 2], "name" => ["John", "Jane"])
        .unwrap()
        .lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_not_have_columns_error_when_columns_same_length() {
    let table = df!("id" => [1i64, 2], "age" => [30i64, 25]).unwrap().lazy();
    let schema = create_schema(json!({
        "allRequired": true,
        "properties": {"id": {"type": "number"}, "name": {"type": "number"}},
    }));
    assert_eq!(inspect(table, &schema, None, None), vec![missing("name")]);
}

#[test]
fn test_should_detect_missing_columns() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    let schema = create_id_name_schema(json!({"allRequired": true}));
    assert_eq!(inspect(table, &schema, None, None), vec![missing("name")]);
}

#[test]
fn test_should_pass_when_column_names_match_regardless_of_order() {
    let table = df!("name" => ["John", "Jane"], "id" => [1i64, 2])
        .unwrap()
        .lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_detect_missing_columns_with_required() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    let schema = create_id_name_schema(json!({"required": ["name"]}));
    assert_eq!(inspect(table, &schema, None, None), vec![missing("name")]);
}

#[test]
fn test_should_pass_when_non_required_columns_are_missing() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_pass_when_data_contains_all_schema_columns() {
    let table = df!("id" => [1i64, 2], "name" => ["John", "Jane"], "age" => [30i64, 25])
        .unwrap()
        .lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_pass_when_at_least_one_column_matches() {
    let table = df!("id" => [1i64, 2], "age" => [30i64, 25]).unwrap().lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_detect_missing_columns_with_all_required() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    let schema = create_id_name_schema(json!({"allRequired": true}));
    assert_eq!(inspect(table, &schema, None, None), vec![missing("name")]);
}

#[test]
fn test_should_detect_when_no_columns_match() {
    let table = df!("age" => [30i64, 25], "email" => ["john@example.com", "jane@example.com"])
        .unwrap()
        .lazy();
    let schema = create_id_name_schema(json!({"allRequired": true}));
    assert_eq!(
        inspect(table, &schema, None, None),
        vec![missing("id"), missing("name")]
    );
}

fn create_bad_table() -> LazyFrame {
    let columns: Vec<Column> = (0..8)
        .map(|index| Column::new(format!("c{index}").into(), ["BAD", "ALSO BAD"]))
        .collect();
    DataFrame::new_infer_height(columns).unwrap().lazy()
}

fn create_bad_schema() -> TableSchema {
    let properties: serde_json::Map<String, Value> = (0..8)
        .map(|index| (format!("c{index}"), json!({"type": "integer"})))
        .collect();
    create_schema(json!({"properties": properties}))
}

#[test]
fn test_should_produce_identical_errors_for_serial_and_concurrent() {
    let schema = create_bad_schema();
    let serial = inspect(create_bad_table(), &schema, None, Some(1));
    let concurrent = inspect(create_bad_table(), &schema, None, Some(4));
    assert_eq!(serial, concurrent);
}

#[test]
fn test_should_truncate_identically_with_max_errors() {
    let schema = create_bad_schema();
    let serial = inspect(create_bad_table(), &schema, Some(5), Some(1));
    let concurrent = inspect(create_bad_table(), &schema, Some(5), Some(4));
    assert_eq!(serial.len(), 5);
    assert_eq!(serial, concurrent);
}

#[test]
fn test_should_keep_missing_column_errors_in_schema_order() {
    let table = df!("b" => [1i64], "d" => [1i64]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {
            "a": {"type": "string"},
            "b": {"type": "string"},
            "c": {"type": "string"},
            "d": {"type": "string"},
            "e": {"type": "string"},
        },
        "allRequired": true,
    }));

    let missing_names: Vec<Value> = inspect(table, &schema, None, Some(4))
        .into_iter()
        .filter(|error| error["type"] == "column/missing")
        .map(|error| error["columnName"].clone())
        .collect();
    assert_eq!(missing_names, vec![json!("a"), json!("c"), json!("e")]);
}

#[test]
fn test_should_report_column_errors_before_cell_errors() {
    let table = df!("a" => ["x"]).unwrap().lazy();
    let schema = create_schema(json!({
        "properties": {"a": {"type": "number"}, "b": {"type": "string"}},
        "allRequired": true,
    }));

    let types: Vec<Value> = inspect(table, &schema, None, None)
        .into_iter()
        .map(|error| error["type"].clone())
        .collect();
    assert_eq!(types, vec![json!("column/missing"), json!("cell/type")]);
}

#[test]
fn test_should_apply_table_level_missing_values() {
    let table = df!("id" => ["1", "NA"]).unwrap().lazy();
    let schema = create_schema(json!({
        "missingValues": ["NA"],
        "properties": {"id": {"type": "number"}},
    }));

    assert_eq!(
        inspect(table, &schema, None, None),
        vec![json!({"type": "cell/missing", "columnName": "id", "rowNumber": 2, "cell": "NA"})]
    );
}

#[test]
fn test_should_pass_when_data_contains_exact_schema_columns() {
    let table = df!("id" => [1i64, 2], "name" => ["John", "Jane"])
        .unwrap()
        .lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_detect_missing_columns_again() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    let schema = create_id_name_schema(json!({"required": ["name"]}));
    assert_eq!(inspect(table, &schema, None, None), vec![missing("name")]);
}

#[test]
fn test_should_pass_when_schema_contains_all_data_columns() {
    let table = df!("id" => [1i64, 2]).unwrap().lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}

#[test]
fn test_should_pass_when_schema_contains_exact_data_columns() {
    let table = df!("id" => [1i64, 2], "name" => ["John", "Jane"])
        .unwrap()
        .lazy();
    let schema = create_id_name_schema(json!({}));
    assert!(inspect(table, &schema, None, None).is_empty());
}
