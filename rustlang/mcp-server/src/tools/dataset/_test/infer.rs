use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::json;

use crate::tools::dataset::infer::{InferDatasetInput, infer_dataset_tool};

#[test]
fn test_should_infer_resource_metadata() {
    let path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();
    let input: InferDatasetInput =
        serde_json::from_value(json!({"dataset": {"resources": [{"data": path}]}})).unwrap();
    let dataset = infer_dataset_tool(input).unwrap().dataset;
    let resource = &dataset.resources.unwrap()[0];
    assert!(resource.file_dialect.is_some());
    assert!(resource.table_schema.is_some());
}

#[test]
fn test_should_preserve_dataset_metadata() {
    let input: InferDatasetInput =
        serde_json::from_value(json!({"dataset": {"version": "1.0.0", "resources": []}})).unwrap();
    let dataset = infer_dataset_tool(input).unwrap().dataset;
    assert_eq!(dataset.datacite.version.as_deref(), Some("1.0.0"));
}
