use fairspec_dataset::write_temp_file;
use fairspec_metadata::Resource;
use serde_json::{Value, json};

use crate::actions::table::validate::validate_table;
use crate::models::table::ValidateTableOptions;

fn create_resource(content: &str, table_schema: Option<Value>) -> Resource {
    let path = write_temp_file(content, false, None, Some("csv")).unwrap();
    let mut descriptor = json!({"data": path});
    if let Some(table_schema) = table_schema {
        descriptor["tableSchema"] = table_schema;
    }
    serde_json::from_value(descriptor).unwrap()
}

fn create_schema() -> Option<Value> {
    Some(json!({"properties": {"id": {"type": "integer"}, "name": {"type": "string"}}}))
}

fn create_options(options: Value) -> ValidateTableOptions {
    serde_json::from_value(options).unwrap()
}

#[test]
fn test_should_validate_valid_table() {
    let resource = create_resource("id,name\n1,english\n2,中文", create_schema());
    assert!(
        validate_table(&resource, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_detect_type_error() {
    let resource = create_resource("id,name\nBAD,english", create_schema());
    let report = validate_table(&resource, &Default::default()).unwrap();
    assert!(!report.valid);
    assert!(!report.errors.is_empty());
}

#[test]
fn test_should_validate_without_schema() {
    let resource = create_resource("id,name\n1,english", None);
    assert!(
        validate_table(&resource, &Default::default())
            .unwrap()
            .valid
    );
}

#[test]
fn test_should_report_rows_failing_a_check() {
    let resource = create_resource("id,start,end\n1,1,2\n2,3,2", None);
    let options = create_options(json!({
        "checks": [{"name": "end-after-start", "condition": "\"end\" >= start"}],
    }));
    let report = validate_table(&resource, &options).unwrap();
    assert!(!report.valid);
    assert_eq!(
        serde_json::to_value(&report.errors).unwrap(),
        json!([{"type": "row/check", "checkName": "end-after-start", "rowNumber": 2}])
    );
}

#[test]
fn test_should_run_checks_when_schema_inference_is_skipped() {
    let resource = create_resource("id,start,end\n1,1,2\n2,3,2", None);
    let options = create_options(json!({
        "noInfer": true,
        "checks": [{"name": "two-rows", "condition": "count(*) = 2"}],
    }));
    assert!(validate_table(&resource, &options).unwrap().valid);
}
