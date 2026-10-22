use fairspec_dataset::write_temp_file;
use fairspec_metadata::{Dataset, FairspecError, ResourceError, TableError};
use serde_json::{Value, json};

use crate::actions::dataset::foreign_key::validate_dataset_foreign_keys;

fn write_csv(content: &str) -> String {
    write_temp_file(content, false, None, Some("csv")).unwrap()
}

fn validate(resources: Value) -> fairspec_metadata::Report {
    let dataset: Dataset = serde_json::from_value(json!({"resources": resources})).unwrap();
    validate_dataset_foreign_keys(&dataset, &Default::default()).unwrap()
}

fn create_names_resource(path: String) -> Value {
    json!({
        "data": path,
        "name": "names",
        "tableSchema": {"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}},
    })
}

fn create_refs_resource(path: String) -> Value {
    json!({
        "data": path,
        "name": "refs",
        "tableSchema": {
            "properties": {"id": {"type": "integer"}, "name_id": {"type": "integer"}},
            "foreignKeys": [{"columns": ["name_id"], "reference": {"resource": "names", "columns": ["id"]}}],
        },
    })
}

#[test]
fn test_should_validate_valid_foreign_keys() {
    let report = validate(json!([
        create_names_resource(write_csv("id,name\n1,english\n2,中文")),
        create_refs_resource(write_csv("id,name_id\n1,1\n2,2")),
    ]));
    assert!(report.valid);
}

#[test]
fn test_should_report_missing_referenced_resource() {
    let report = validate(json!([create_refs_resource(write_csv("id,name_id\n1,1"))]));
    assert_eq!(report.errors.len(), 1);
    let FairspecError::Resource(ResourceError::Missing(error)) = &report.errors[0] else {
        panic!(
            "expected a resource/missing error, got {:?}",
            report.errors[0]
        );
    };
    assert_eq!(error.resource_name.as_deref(), Some("names"));
    assert_eq!(error.referencing_resource_name, "refs");
}

#[test]
fn test_should_check_self_references_and_skip_nulls() {
    let report = validate(json!([{
        "data": write_csv("id,parent_id\n1,\n2,1\n3,9"),
        "name": "tree",
        "tableSchema": {
            "properties": {"id": {"type": "integer"}, "parent_id": {"type": "integer"}},
            "foreignKeys": [{"columns": ["parent_id"], "reference": {"columns": ["id"]}}],
        },
    }]));
    assert_eq!(report.errors.len(), 1);
    let FairspecError::Table(TableError::ForeignKey(error)) = &report.errors[0] else {
        panic!("expected a foreignKey error, got {:?}", report.errors[0]);
    };
    assert_eq!(error.cells, ["9"]);
}

#[test]
fn test_should_detect_foreign_key_violation() {
    let report = validate(json!([
        create_names_resource(write_csv("id,name\n1,english\n2,中文")),
        create_refs_resource(write_csv("id,name_id\n1,1\n2,999")),
    ]));
    assert!(!report.valid);
    assert!(!report.errors.is_empty());
}

#[test]
fn test_should_handle_no_foreign_keys() {
    let report = validate(json!([{
        "data": write_csv("id,name\n1,english"),
        "name": "test",
        "tableSchema": {"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}},
    }]));
    assert!(report.valid);
}
