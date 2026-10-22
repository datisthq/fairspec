use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::actions::data::load::load_data;

fn create_resource(descriptor: Value) -> Resource {
    serde_json::from_value(descriptor).unwrap()
}

fn write_file(content: &str, format: &str) -> String {
    write_temp_file(content, false, None, Some(format)).unwrap()
}

#[test]
fn test_should_return_inline_data() {
    let resource = create_resource(json!({"data": [{"id": 1}, {"id": 2}]}));
    assert_eq!(
        load_data(&resource).unwrap(),
        Some(json!([{"id": 1}, {"id": 2}]))
    );
}

#[test]
fn test_should_return_inline_object() {
    let resource = create_resource(json!({"data": {"key": "value"}}));
    assert_eq!(load_data(&resource).unwrap(), Some(json!({"key": "value"})));
}

#[test]
fn test_should_load_json_file() {
    let path = write_file(r#"{"key": "value"}"#, "json");
    let resource = create_resource(json!({"data": path}));
    assert_eq!(load_data(&resource).unwrap(), Some(json!({"key": "value"})));
}

#[test]
fn test_should_load_json_file_with_top_level_array() {
    let data = json!([{"id": 1, "name": "A"}, {"id": 2, "name": "B"}]);
    let path = write_file(&data.to_string(), "json");
    let resource = create_resource(json!({"data": path}));
    assert_eq!(load_data(&resource).unwrap(), Some(data));
}

#[test]
fn test_should_return_none_for_non_json_file() {
    let path = write_file("id,name\n1,english", "csv");
    let resource = create_resource(json!({"data": path}));
    assert_eq!(load_data(&resource).unwrap(), None);
}

#[test]
fn test_should_return_none_for_empty_resource() {
    assert_eq!(load_data(&Resource::default()).unwrap(), None);
}
