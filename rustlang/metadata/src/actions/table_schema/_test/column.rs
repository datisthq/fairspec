use serde_json::{Value, json};

use crate::actions::table_schema::column::get_columns;
use crate::models::column::column::{Column, ColumnType};
use crate::models::table_schema::TableSchema;

fn get_columns_from(table_schema: Value) -> Vec<Column> {
    let Value::Object(table_schema) = table_schema else {
        panic!("table schema must be an object");
    };
    get_columns(&table_schema).unwrap()
}

#[test]
fn test_property_without_type_is_unknown() {
    let table_schema: TableSchema =
        serde_json::from_value(json!({"properties": {"x": {"title": "Column"}}})).unwrap();
    let columns = get_columns_from(serde_json::to_value(table_schema).unwrap());
    let types: Vec<ColumnType> = columns.iter().map(Column::column_type).collect();
    assert_eq!(types, [ColumnType::Unknown]);
}

#[test]
fn test_sets_required_for_listed_columns() {
    let columns = get_columns_from(json!({
        "required": ["id"],
        "properties": {"id": {"type": "number"}, "name": {"type": "string"}},
    }));
    assert_eq!(columns[0].name(), "id");
    assert_eq!(columns[0].required(), Some(true));
    assert_eq!(columns[1].name(), "name");
    assert_eq!(columns[1].required(), None);
}

#[test]
fn test_sets_required_for_all_with_all_required() {
    let columns = get_columns_from(json!({
        "allRequired": true,
        "properties": {"id": {"type": "number"}, "name": {"type": "string"}},
    }));
    assert!(columns.iter().all(|column| column.required() == Some(true)));
}

#[test]
fn test_sets_nullable_for_nullable_types() {
    let columns = get_columns_from(json!({"properties": {"id": {"type": ["number", "null"]}}}));
    assert_eq!(columns.len(), 1);
    assert_eq!(columns[0].name(), "id");
    assert_eq!(columns[0].nullable(), Some(true));
}

#[test]
fn test_sets_both_required_and_nullable() {
    let columns = get_columns_from(json!({
        "allRequired": true,
        "properties": {"id": {"type": ["number", "null"]}},
    }));
    assert_eq!(columns.len(), 1);
    assert_eq!(columns[0].required(), Some(true));
    assert_eq!(columns[0].nullable(), Some(true));
}

#[test]
fn test_property_with_null_fields_is_unknown() {
    let columns = get_columns_from(json!({"properties": {"x": {"type": null, "title": "Column"}}}));
    assert_eq!(columns[0].column_type(), ColumnType::Unknown);
}
