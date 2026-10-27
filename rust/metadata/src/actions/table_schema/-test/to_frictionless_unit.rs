use crate::models::descriptor::Descriptor;
use serde_json::{Value, json};

use crate::actions::table_schema::to_frictionless::convert_table_schema_to_frictionless;

fn convert(table_schema: Value) -> Value {
    let table_schema: Descriptor = serde_json::from_value(table_schema).unwrap();
    Value::Object(convert_table_schema_to_frictionless(&table_schema).unwrap())
}

#[test]
fn test_column_without_null_is_required() {
    let result = convert(json!({"properties": {
        "id": {"type": "integer", "minimum": 0},
        "name": {"type": ["string", "null"], "maxLength": 3},
        "any": {},
    }}));
    assert_eq!(
        result,
        json!({"fields": [
            {"name": "id", "type": "integer", "constraints": {"required": true, "minimum": 0}},
            {"name": "name", "type": "string", "constraints": {"maxLength": 3}},
            {"name": "any", "type": "any"},
        ]})
    );
}

#[test]
fn test_appends_empty_missing_value() {
    let result = convert(json!({"properties": {}, "missingValues": ["NA"]}));
    assert_eq!(result["missingValues"], json!(["NA", ""]));

    let result = convert(json!({"properties": {}, "missingValues": [""]}));
    assert!(result.get("missingValues").is_none());
}

#[test]
fn test_maps_types_frictionless_lacks() {
    let result = convert(json!({"properties": {
        "h": {"type": "string", "format": "hex"},
        "d": {"type": "string", "format": "decimal", "decimalChar": ","},
        "l": {"type": "string", "format": "list", "itemType": "local-time", "minItems": 1},
        "n": {"type": ["number", "null"], "enum": [1.5, null]},
    }}));
    assert_eq!(
        result["fields"],
        json!([
            {"name": "h", "type": "string", "constraints": {"required": true}},
            {"name": "d", "type": "number", "decimalChar": ",", "constraints": {"required": true}},
            {"name": "l", "type": "list", "itemType": "time", "constraints": {"required": true, "minLength": 1}},
            {"name": "n", "type": "number", "constraints": {"enum": [1.5]}},
        ])
    );
}

#[test]
fn test_reference_resource_defaults_to_self() {
    let result = convert(json!({
        "properties": {"a": {"type": "integer"}},
        "foreignKeys": [{"columns": ["a"], "reference": {"columns": ["b"]}}],
    }));
    assert_eq!(
        result["foreignKeys"],
        json!([{"fields": ["a"], "reference": {"resource": "", "fields": ["b"]}}])
    );
}

#[test]
fn test_nullable_integer_categorical_stays_integer() {
    let result = convert(json!({"properties": {
        "c": {"type": ["integer", "null"], "format": "categorical", "categories": [1]},
    }}));
    assert_eq!(
        result["fields"],
        json!([{"name": "c", "type": "integer", "categories": [1]}])
    );
}
