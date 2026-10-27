use serde_json::{Value, json};

use crate::actions::dataset::validate::validate_dataset_descriptor;
use crate::actions::descriptor::inspect::inspect_model;
use crate::models::dataset::Dataset;
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::table_schema::TableSchema;

fn create_descriptor(value: Value) -> Descriptor {
    let Value::Object(descriptor) = value else {
        unreachable!()
    };
    descriptor
}

#[test]
fn test_returns_the_parsed_model() {
    let descriptor = create_descriptor(json!({"properties": {"id": {"type": "integer"}}}));
    let (table_schema, errors) = inspect_model::<TableSchema>(&descriptor, None);
    assert!(errors.is_empty());
    assert!(table_schema.is_some());
}

#[test]
fn test_reports_a_mismatch_at_its_pointer() {
    let descriptor =
        create_descriptor(json!({"properties": {"x": {"type": "integer", "minimum": "low"}}}));
    let (table_schema, errors) =
        inspect_model::<TableSchema>(&descriptor, Some("/resources/0/tableSchema"));
    assert!(table_schema.is_none());
    let [FairspecError::Metadata(error)] = errors.as_slice() else {
        panic!("expected one metadata error, got {errors:?}")
    };
    assert!(
        error
            .json_pointer
            .starts_with("/resources/0/tableSchema/properties/x")
    );
}

#[test]
fn test_keeps_extension_properties() {
    let descriptor = create_descriptor(
        json!({"resources": [{"data": "data.csv", "spectralRange": {"min": 400}}]}),
    );
    let result = validate_dataset_descriptor(descriptor, None).unwrap();
    assert!(result.valid);
    let dataset: Dataset = result.dataset.unwrap();
    let output = serde_json::to_value(&dataset).unwrap();
    assert_eq!(output["resources"][0]["spectralRange"], json!({"min": 400}));
}
