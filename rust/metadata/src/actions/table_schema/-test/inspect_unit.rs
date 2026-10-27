use serde_json::{Value, json};

use crate::actions::table_schema::inspect::{inspect_table_schema, inspect_table_schema_nulls};
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::table_schema::TableSchema;

fn inspect(table_schema: Value) -> Value {
    let table_schema: TableSchema = serde_json::from_value(table_schema).unwrap();
    serde_json::to_value(inspect_table_schema(&table_schema, None)).unwrap()
}

fn get_pointers(errors: &[FairspecError]) -> Vec<String> {
    let errors = serde_json::to_value(errors).unwrap();
    errors
        .as_array()
        .unwrap()
        .iter()
        .map(|error| error["jsonPointer"].as_str().unwrap().to_string())
        .collect()
}

fn create_descriptor(value: Value) -> Descriptor {
    serde_json::from_value(value).unwrap()
}

#[test]
fn test_required_column_not_in_properties() {
    let errors = inspect(json!({
        "required": ["id", "nope"],
        "properties": {"id": {"type": "integer"}},
    }));
    assert_eq!(
        errors,
        json!([{
            "type": "metadata",
            "message": "column \"nope\" is not in properties",
            "jsonPointer": "/required/1",
        }])
    );
}

#[test]
fn test_pattern_not_a_regular_expression() {
    let errors = inspect(json!({"properties": {"a/b": {"type": "string", "pattern": "[a-"}}}));
    assert_eq!(
        errors,
        json!([{
            "type": "metadata",
            "message": "pattern \"[a-\" is not a valid regular expression",
            "jsonPointer": "/properties/a~1b/pattern",
        }])
    );
}

#[test]
fn test_valid_pattern() {
    let errors = inspect(json!({
        "properties": {"code": {"type": "string", "pattern": "^[A-Z]{3}-[0-9]{4}$"}},
    }));
    assert_eq!(errors, json!([]));
}

#[test]
fn test_null_values_on_column_without_null() {
    let descriptor = create_descriptor(json!({
        "properties": {"x": {
            "type": "string",
            "const": null,
            "enum": ["a", null],
            "default": null,
            "examples": [null],
        }},
    }));
    let errors = inspect_table_schema_nulls(&descriptor, Some("/resources/0/tableSchema"));
    let pointer = "/resources/0/tableSchema/properties/x";
    assert_eq!(
        get_pointers(&errors),
        [
            format!("{pointer}/enum/1"),
            format!("{pointer}/const"),
            format!("{pointer}/default"),
            format!("{pointer}/examples/0"),
        ]
    );
    let first = serde_json::to_value(&errors[0]).unwrap();
    assert_eq!(
        first["message"],
        "null is not allowed: the type of column \"x\" does not include \"null\""
    );
}

#[test]
fn test_null_values_on_nullable_or_unknown_column() {
    let descriptor = create_descriptor(json!({
        "properties": {
            "x": {"type": ["string", "null"], "const": null, "enum": ["a", null]},
            "y": {"const": null},
        },
    }));
    assert!(inspect_table_schema_nulls(&descriptor, None).is_empty());
}
