use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::actions::file_dialect::infer::infer_file_dialect;

fn infer(descriptor: Value) -> Option<Value> {
    let resource: Resource = serde_json::from_value(descriptor).unwrap();
    infer_file_dialect(&resource, &Default::default())
        .unwrap()
        .map(|dialect| serde_json::to_value(dialect).unwrap())
}

fn write_file(content: &str, format: &str) -> String {
    write_temp_file(content, false, None, Some(format)).unwrap()
}

#[test]
fn test_should_infer_csv_dialect() {
    let path = write_file("id,name\n1,english\n2,中文", "csv");
    assert!(infer(json!({"data": path})).is_some());
}

#[test]
fn test_should_infer_json_dialect() {
    let path = write_file(r#"[{"id": 1}]"#, "json");
    assert_eq!(infer(json!({"data": path})).unwrap()["format"], "json");
}

#[test]
fn test_should_infer_jsonl_dialect() {
    let path = write_file("{\"id\": 1}\n{\"id\": 2}\n", "jsonl");
    assert_eq!(infer(json!({"data": path})).unwrap()["format"], "jsonl");
}

#[test]
fn test_should_infer_xlsx_dialect() {
    assert!(infer(json!({"data": "test.xlsx"})).is_some());
}

#[test]
fn test_should_return_none_for_unknown_format() {
    assert!(infer(json!({"data": "test.unknown"})).is_none());
}

#[test]
fn test_should_return_none_for_no_data() {
    assert!(infer(json!({})).is_none());
}
