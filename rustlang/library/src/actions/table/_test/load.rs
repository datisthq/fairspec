use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use fairspec_table::Table;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::table::load::load_table;

fn load(descriptor: Value) -> Option<Table> {
    let resource: Resource = serde_json::from_value(descriptor).unwrap();
    load_table(&resource, &Default::default()).unwrap()
}

fn assert_rows(table: Table) {
    let frame = table.collect().unwrap();
    let ids = frame.column("id").unwrap().cast(&DataType::Int64).unwrap();
    let ids = ids.i64().unwrap();
    let names = frame.column("name").unwrap().str().unwrap().clone();
    let ids: Vec<Option<i64>> = (0..ids.len()).map(|index| ids.get(index)).collect();
    let names: Vec<Option<&str>> = (0..names.len()).map(|index| names.get(index)).collect();
    assert_eq!(ids, [Some(1), Some(2)]);
    assert_eq!(names, [Some("english"), Some("中文")]);
}

#[test]
fn test_should_load_csv_table() {
    let path = write_temp_file("id,name\n1,english\n2,中文", false, None, Some("csv")).unwrap();
    assert_rows(load(json!({"data": path})).unwrap());
}

#[test]
fn test_should_load_inline_table() {
    let data = json!([{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}]);
    assert_rows(load(json!({"data": data})).unwrap());
}

#[test]
fn test_should_return_none_for_empty_resource() {
    assert!(load(json!({})).is_none());
}

#[test]
fn test_should_load_json_table() {
    let path = write_temp_file(
        r#"[{"id": 1, "name": "english"}]"#,
        false,
        None,
        Some("json"),
    )
    .unwrap();
    assert!(load(json!({"data": path})).is_some());
}
