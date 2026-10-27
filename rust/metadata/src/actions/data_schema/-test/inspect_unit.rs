use serde_json::{Value, json};

use crate::actions::data_schema::inspect::inspect_data_schema;
use crate::models::descriptor::Descriptor;

fn to_map(value: Value) -> Descriptor {
    serde_json::from_value(value).unwrap()
}

fn get_pointers(descriptor: Value, root_json_pointer: Option<&str>) -> Vec<String> {
    inspect_data_schema(&to_map(descriptor), root_json_pointer)
        .into_iter()
        .map(|issue| issue.json_pointer)
        .collect()
}

#[test]
fn test_accepts_the_2020_12_dialect_and_a_fairspec_profile() {
    for schema in [
        "https://json-schema.org/draft/2020-12/schema",
        "https://fairspec.org/profiles/1.0.0/data-schema.json",
    ] {
        assert!(get_pointers(json!({"$schema": schema}), None).is_empty());
    }
}

#[test]
fn test_reports_another_dialect_at_schema() {
    let pointers = get_pointers(
        json!({"$schema": "http://json-schema.org/draft-07/schema#"}),
        Some("/dataSchema"),
    );
    assert_eq!(pointers, vec!["/dataSchema/$schema"]);
}

#[test]
fn test_accepts_references_that_resolve_within_the_document() {
    let pointers = get_pointers(
        json!({
            "$defs": {"a/b": {"type": "string"}, "c d": {"$anchor": "cd"}},
            "items": [{"$ref": "#/$defs/a~1b"}, {"$ref": "#/$defs/c%20d"}, {"$ref": "#cd"}],
            "not": {"$ref": "#"},
        }),
        None,
    );
    assert!(pointers.is_empty());
}

#[test]
fn test_reports_every_reference_that_leaves_the_document_in_order() {
    let pointers = get_pointers(
        json!({
            "properties": {
                "a": {"$ref": "https://example.com/schema.json"},
                "b": {"$ref": "#/$defs/missing"},
            },
            "items": {"$ref": "#/$defs/%ZZ"},
            "const": {"$ref": "https://example.com/data-not-schema"},
        }),
        None,
    );
    assert_eq!(
        pointers,
        vec!["/properties/a/$ref", "/properties/b/$ref", "/items/$ref"]
    );
}

#[test]
fn test_reports_an_index_beyond_any_int() {
    let reference = format!("#/$defs/a/{}", "9".repeat(5000));
    let pointers = get_pointers(
        json!({"$defs": {"a": [{}]}, "not": {"$ref": reference}}),
        None,
    );
    assert_eq!(pointers, vec!["/not/$ref"]);
}
