use crate::models::descriptor::Descriptor;
use serde_json::{Value, json};

use crate::actions::table_schema::from_frictionless::convert_table_schema_from_frictionless;

fn convert(schema: Value) -> Value {
    let schema: Descriptor = serde_json::from_value(schema).unwrap();
    Value::Object(convert_table_schema_from_frictionless(&schema, None))
}

#[test]
fn test_field_allows_null_unless_required() {
    let result = convert(json!({"fields": [
        {"name": "id", "type": "integer", "constraints": {"required": true}},
        {"name": "name"},
    ]}));
    assert_eq!(
        result,
        json!({"properties": {
            "id": {"type": "integer"},
            "name": {"type": ["string", "null"]},
        }})
    );
}

#[test]
fn test_drops_default_missing_values() {
    let result = convert(json!({"fields": [], "missingValues": [""]}));
    assert_eq!(result, json!({"properties": {}}));

    let result = convert(json!({"fields": [], "missingValues": ["", "NA"]}));
    assert_eq!(result["missingValues"], json!(["", "NA"]));
}

#[test]
fn test_maps_types_and_formats() {
    let result = convert(json!({"fields": [
        {"name": "t", "type": "time", "format": "%H:%M%z"},
        {"name": "dt", "type": "datetime", "format": "fmt:%Y"},
        {"name": "y", "type": "year", "constraints": {"minimum": 1.0}},
        {"name": "g", "type": "geopoint", "format": "array"},
        {"name": "x", "type": "any"},
    ]}));
    assert_eq!(
        result["properties"],
        json!({
            "t": {"type": ["string", "null"], "format": "time", "temporalFormat": "%H:%M%z"},
            "dt": {"type": ["string", "null"], "format": "local-date-time", "temporalFormat": "%Y"},
            "y": {"type": ["integer", "null"], "minimum": 1.0},
            "g": {"type": ["array", "null"]},
            "x": {},
        })
    );
}

#[test]
fn test_converts_keys_and_reference_names() {
    let result = convert(json!({
        "fields": [{"name": "a", "constraints": {"unique": true}}, {"name": "b"}],
        "primaryKey": "a",
        "uniqueKeys": [["a", "b"]],
        "foreignKeys": [{"fields": "b", "reference": {"resource": "other-table", "fields": "id"}}],
    }));
    assert_eq!(result["primaryKey"], json!(["a"]));
    assert_eq!(result["uniqueKeys"], json!([["a", "b"], ["a"]]));
    assert_eq!(
        result["foreignKeys"],
        json!([{"columns": ["b"], "reference": {"resource": "other_table", "columns": ["id"]}}])
    );
}
