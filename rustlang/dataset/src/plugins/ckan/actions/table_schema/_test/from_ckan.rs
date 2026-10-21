use serde_json::{Value, json};

use crate::plugins::ckan::actions::table_schema::from_ckan::convert_table_schema_from_ckan;
use crate::plugins::ckan::models::schema::CkanSchema;

pub(super) fn load_fixture() -> CkanSchema {
    let text = include_str!("fixtures/ckan-schema.json");
    serde_json::from_str(text).unwrap()
}

/// The converted properties as JSON, so assertions read like the Python twin's.
fn convert(ckan_schema: &CkanSchema) -> Value {
    let table_schema = convert_table_schema_from_ckan(ckan_schema);
    serde_json::to_value(table_schema.properties.unwrap_or_default()).unwrap()
}

#[test]
fn test_converts_ckan_schema_to_fairspec_table_schema() {
    let ckan_schema = load_fixture();
    let properties = convert(&ckan_schema);

    assert_eq!(
        properties.as_object().unwrap().len(),
        ckan_schema.fields.len()
    );

    assert_eq!(properties["id"]["type"], "integer");
    assert_eq!(properties["id"]["title"], "ID");
    assert_eq!(properties["id"]["description"], "Unique identifier");

    assert_eq!(properties["name"]["type"], "string");
    assert_eq!(properties["name"]["title"], "Name");
    assert_eq!(properties["name"]["description"], "Person's full name");

    assert_eq!(properties["age"]["type"], "integer");
    assert_eq!(properties["age"].get("title"), None);
    assert_eq!(properties["age"].get("description"), None);

    assert_eq!(properties["score"]["type"], "number");
    assert_eq!(properties["score"]["title"], "Score");
    assert_eq!(properties["score"]["description"], "Test score");

    assert_eq!(properties["is_active"]["type"], "boolean");

    assert_eq!(properties["birth_date"]["type"], "string");
    assert_eq!(properties["birth_date"]["format"], "date");
    assert_eq!(properties["birth_date"]["title"], "Birth Date");
    assert_eq!(properties["birth_date"]["description"], "Date of birth");

    assert_eq!(properties["start_time"]["type"], "string");
    assert_eq!(properties["start_time"]["format"], "local-time");

    assert_eq!(properties["created_at"]["type"], "string");
    assert_eq!(properties["created_at"]["format"], "local-date-time");
    assert_eq!(properties["created_at"]["title"], "Created At");
    assert_eq!(
        properties["created_at"]["description"],
        "Timestamp when record was created"
    );

    assert_eq!(properties["metadata"]["type"], "object");

    assert_eq!(properties["tags"]["type"], "array");
    assert_eq!(properties["tags"]["title"], "Tags");
    assert_eq!(properties["tags"]["description"], "List of tags");
}

#[test]
fn test_converts_ckan_type_aliases() {
    let properties = convert(&load_fixture());

    assert_eq!(properties["string_field"]["type"], "string");
    assert_eq!(properties["integer_field"]["type"], "integer");
    assert_eq!(properties["number_field"]["type"], "number");
    assert_eq!(properties["float_field"]["type"], "number");
    assert_eq!(properties["boolean_field"]["type"], "boolean");
    assert_eq!(properties["datetime_field"]["type"], "string");
    assert_eq!(properties["datetime_field"]["format"], "local-date-time");
    assert_eq!(properties["object_field"]["type"], "object");
}

#[test]
fn test_handles_unknown_field_types() {
    let properties = convert(&load_fixture());
    assert_eq!(properties["unknown_field"]["type"], "string");
}

#[test]
fn test_respects_type_override_in_field_info() {
    let properties = convert(&load_fixture());
    assert_eq!(properties["override_field"]["type"], "integer");
    assert_eq!(properties["override_field"]["title"], "Override Field");
    assert_eq!(
        properties["override_field"]["description"],
        "Field with type override"
    );
}

#[test]
fn test_handles_empty_fields_array() {
    assert_eq!(convert(&CkanSchema { fields: vec![] }), json!({}));
}

#[test]
fn test_handles_fields_without_info_object() {
    let ckan_schema = serde_json::from_value(json!({
        "fields": [{"id": "simple_field", "type": "text"}]
    }))
    .unwrap();
    assert_eq!(
        convert(&ckan_schema),
        json!({"simple_field": {"type": "string"}})
    );
}

#[test]
fn test_handles_case_insensitive_type_conversion() {
    let ckan_schema = serde_json::from_value(json!({
        "fields": [
            {"id": "field1", "type": "TEXT"},
            {"id": "field2", "type": "INT"},
            {"id": "field3", "type": "BOOL"},
            {"id": "field4", "type": "TIMESTAMP"},
        ]
    }))
    .unwrap();
    let properties = convert(&ckan_schema);

    assert_eq!(properties["field1"]["type"], "string");
    assert_eq!(properties["field2"]["type"], "integer");
    assert_eq!(properties["field3"]["type"], "boolean");
    assert_eq!(properties["field4"]["type"], "string");
    assert_eq!(properties["field4"]["format"], "local-date-time");
}
