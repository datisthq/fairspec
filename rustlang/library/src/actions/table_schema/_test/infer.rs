use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::json;

use crate::actions::table_schema::infer::infer_table_schema;

#[test]
fn test_should_infer_schema_from_csv() {
    let path = write_temp_file("id,name\n1,english\n2,中文", false, None, Some("csv")).unwrap();
    let resource: Resource = serde_json::from_value(json!({"data": path})).unwrap();
    let schema = infer_table_schema(&resource, &Default::default()).unwrap();
    assert!(schema.unwrap().properties.is_some());
}

#[test]
fn test_should_return_none_for_empty_resource() {
    let schema = infer_table_schema(&Resource::default(), &Default::default()).unwrap();
    assert!(schema.is_none());
}

#[test]
fn test_should_infer_schema_from_inline_data() {
    let resource: Resource = serde_json::from_value(json!({
        "data": [{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}],
    }))
    .unwrap();
    let schema = infer_table_schema(&resource, &Default::default()).unwrap();
    assert!(schema.is_some());
}
