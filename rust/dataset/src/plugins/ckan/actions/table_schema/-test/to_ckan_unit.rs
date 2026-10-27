use fairspec_metadata::TableSchema;
use serde_json::{Value, json};

use super::from_ckan_unit::load_fixture;
use crate::plugins::ckan::actions::table_schema::from_ckan::convert_table_schema_from_ckan;
use crate::plugins::ckan::actions::table_schema::to_ckan::convert_table_schema_to_ckan;
use crate::plugins::ckan::models::field::CkanField;

fn create_table_schema(value: Value) -> TableSchema {
    serde_json::from_value(value).unwrap()
}

fn find_field<'a>(fields: &'a [CkanField], id: &str) -> &'a CkanField {
    fields.iter().find(|field| field.id == id).unwrap()
}

#[test]
fn test_converts_fairspec_schema_to_ckan_schema() {
    let schema = create_table_schema(json!({
        "properties": {
            "id": {"type": "integer", "title": "ID", "description": "Unique identifier"},
            "name": {"type": "string", "title": "Name", "description": "Person's full name"},
            "age": {"type": "integer"},
            "score": {"type": "number", "title": "Score", "description": "Test score"},
            "is_active": {"type": "boolean"},
            "birth_date": {
                "type": "string", "format": "date",
                "title": "Birth Date", "description": "Date of birth"
            },
            "start_time": {"type": "string", "format": "time"},
            "created_at": {
                "type": "string", "format": "date-time",
                "title": "Created At", "description": "Timestamp when record was created"
            },
            "metadata": {"type": "object"},
            "tags": {"type": "array", "title": "Tags", "description": "List of tags"},
        }
    }));

    let fields = convert_table_schema_to_ckan(&schema).unwrap().fields;
    assert_eq!(fields.len(), 10);

    let check = |id: &str, r#type: &str, label: Option<&str>, notes: Option<&str>| {
        let field = find_field(&fields, id);
        assert_eq!(field.r#type, r#type, "{id}");
        match field.info.as_ref() {
            None => assert!(label.is_none() && notes.is_none(), "{id}"),
            Some(info) => {
                assert_eq!(info.label.as_deref(), label, "{id}");
                assert_eq!(info.notes.as_deref(), notes, "{id}");
                assert_eq!(info.type_override.as_deref(), Some(r#type), "{id}");
            }
        }
    };

    check("id", "int", Some("ID"), Some("Unique identifier"));
    check("name", "text", Some("Name"), Some("Person's full name"));
    check("age", "int", None, None);
    check("score", "numeric", Some("Score"), Some("Test score"));
    check("is_active", "bool", None, None);
    check(
        "birth_date",
        "date",
        Some("Birth Date"),
        Some("Date of birth"),
    );
    check("start_time", "time", None, None);
    check(
        "created_at",
        "timestamptz",
        Some("Created At"),
        Some("Timestamp when record was created"),
    );
    check("metadata", "json", None, None);
    check("tags", "array", Some("Tags"), Some("List of tags"));
}

#[test]
fn test_handles_columns_with_only_title() {
    let schema = create_table_schema(
        json!({"properties": {"field1": {"type": "string", "title": "Field 1"}}}),
    );
    let fields = convert_table_schema_to_ckan(&schema).unwrap().fields;

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].id, "field1");
    assert_eq!(fields[0].r#type, "text");
    let info = fields[0].info.as_ref().unwrap();
    assert_eq!(info.label.as_deref(), Some("Field 1"));
    assert_eq!(info.notes, None);
    assert_eq!(info.type_override.as_deref(), Some("text"));
}

#[test]
fn test_handles_columns_with_only_description() {
    let schema = create_table_schema(json!({
        "properties": {"field1": {"type": "string", "description": "Field 1 description"}}
    }));
    let fields = convert_table_schema_to_ckan(&schema).unwrap().fields;

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].id, "field1");
    assert_eq!(fields[0].r#type, "text");
    let info = fields[0].info.as_ref().unwrap();
    assert_eq!(info.label, None);
    assert_eq!(info.notes.as_deref(), Some("Field 1 description"));
    assert_eq!(info.type_override.as_deref(), Some("text"));
}

#[test]
fn test_handles_columns_without_title_or_description() {
    let schema = create_table_schema(json!({"properties": {"simple_field": {"type": "string"}}}));
    let fields = convert_table_schema_to_ckan(&schema).unwrap().fields;

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].id, "simple_field");
    assert_eq!(fields[0].r#type, "text");
    assert_eq!(fields[0].info, None);
}

#[test]
fn test_handles_empty_properties() {
    let schema = create_table_schema(json!({"properties": {}}));
    assert_eq!(
        convert_table_schema_to_ckan(&schema).unwrap().fields,
        vec![]
    );
}

#[test]
fn test_converts_unmapped_types_to_text() {
    let schema = create_table_schema(json!({"properties": {"null_field": {"type": "null"}}}));
    let fields = convert_table_schema_to_ckan(&schema).unwrap().fields;

    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].r#type, "text");
}

#[test]
fn test_round_trip_ckan_to_fairspec_to_ckan() {
    let original = load_fixture();

    let fairspec_schema = convert_table_schema_from_ckan(&original);
    let result = convert_table_schema_to_ckan(&fairspec_schema).unwrap();

    assert_eq!(result.fields.len(), original.fields.len());
    for original_field in &original.fields {
        let result_field = find_field(&result.fields, &original_field.id);
        if let Some(original_info) = &original_field.info {
            let result_info = result_field.info.as_ref().unwrap();
            if original_info
                .label
                .as_deref()
                .is_some_and(|label| !label.is_empty())
            {
                assert_eq!(result_info.label, original_info.label);
            }
            if original_info
                .notes
                .as_deref()
                .is_some_and(|notes| !notes.is_empty())
            {
                assert_eq!(result_info.notes, original_info.notes);
            }
        }
    }
}
