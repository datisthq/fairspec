use serde_json::{Value, json};

use crate::actions::profile::assert::assert_profile;
use crate::models::exception::Error;
use crate::models::json_schema::JsonSchema;
use crate::models::profile::ProfileType;

fn create_schema(value: Value) -> JsonSchema {
    serde_json::from_value(value).unwrap()
}

#[test]
fn test_valid_dataset_profile() {
    let schema = create_schema(json!({"type": "object"}));
    let result = assert_profile(
        schema.clone(),
        "https://fairspec.org/profiles/latest/dataset.json",
        ProfileType::Dataset,
    );
    assert_eq!(result.unwrap(), schema);
}

#[test]
fn test_valid_versioned_profile() {
    let schema = create_schema(json!({"type": "object"}));
    let result = assert_profile(
        schema.clone(),
        "https://fairspec.org/profiles/1.0.0/catalog.json",
        ProfileType::Catalog,
    );
    assert_eq!(result.unwrap(), schema);
}

#[test]
fn test_valid_profile_via_allof() {
    let schema = create_schema(json!({
        "type": "object",
        "allOf": ["https://fairspec.org/profiles/latest/table-schema.json"],
    }));
    let result = assert_profile(
        schema.clone(),
        "https://example.com/custom.json",
        ProfileType::TableSchema,
    );
    assert_eq!(result.unwrap(), schema);
}

#[test]
fn test_data_schema_accepts_json_schema_draft() {
    let schema = create_schema(json!({"type": "object"}));
    let result = assert_profile(
        schema.clone(),
        "https://json-schema.org/draft/2020-12/schema",
        ProfileType::DataSchema,
    );
    assert_eq!(result.unwrap(), schema);
}

#[test]
fn test_valid_file_dialect_profile() {
    let schema = create_schema(json!({"type": "object"}));
    let result = assert_profile(
        schema.clone(),
        "https://fairspec.org/profiles/latest/file-dialect.json",
        ProfileType::FileDialect,
    );
    assert_eq!(result.unwrap(), schema);
}

#[test]
fn test_wrong_type_in_path() {
    let error = assert_profile(
        create_schema(json!({"type": "object"})),
        "https://fairspec.org/profiles/latest/catalog.json",
        ProfileType::Dataset,
    )
    .unwrap_err();
    assert!(error.to_string().contains("not a valid dataset profile"));
}

#[test]
fn test_wrong_domain() {
    let result = assert_profile(
        create_schema(json!({"type": "object"})),
        "https://other.org/profiles/latest/dataset.json",
        ProfileType::Dataset,
    );
    assert!(matches!(result, Err(Error::Profile(_))));
}

#[test]
fn test_invalid_version_format() {
    let result = assert_profile(
        create_schema(json!({"type": "object"})),
        "https://fairspec.org/profiles/v1/dataset.json",
        ProfileType::Dataset,
    );
    assert!(matches!(result, Err(Error::Profile(_))));
}

#[test]
fn test_json_schema_draft_only_for_data_schema() {
    let result = assert_profile(
        create_schema(json!({"type": "object"})),
        "https://json-schema.org/draft/2020-12/schema",
        ProfileType::Dataset,
    );
    assert!(matches!(result, Err(Error::Profile(_))));
}

#[test]
fn test_valid_profile_via_allof_ref() {
    let schema = create_schema(json!({
        "allOf": [
            {"$ref": "https://fairspec.org/profiles/1.0.0/dataset.json"},
            {"$ref": "#/$defs/Dataset"}
        ]
    }));
    let result = assert_profile(
        schema.clone(),
        "https://example.com/extension/dataset.json",
        ProfileType::Dataset,
    );
    assert_eq!(result.unwrap(), schema);
}

#[test]
fn test_allof_ref_without_official_profile() {
    let schema = create_schema(json!({"allOf": [{"$ref": "#/$defs/Dataset"}]}));
    let result = assert_profile(
        schema,
        "https://example.com/extension/dataset.json",
        ProfileType::Dataset,
    );
    assert!(matches!(result, Err(Error::Profile(_))));
}
