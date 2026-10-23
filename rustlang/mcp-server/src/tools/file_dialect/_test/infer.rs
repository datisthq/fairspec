use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::{Value, json};

use crate::tools::file_dialect::infer::{InferFileDialectInput, infer_file_dialect_tool};

fn infer(resource: Value) -> Value {
    let input: InferFileDialectInput =
        serde_json::from_value(json!({"resource": resource})).unwrap();
    serde_json::to_value(infer_file_dialect_tool(input).unwrap()).unwrap()
}

#[test]
fn test_should_infer_csv_dialect() {
    let path = write_temp_file("id;name\n1;alice\n2;bob\n", false, None, Some("csv")).unwrap();
    let result = infer(json!({"data": path}));
    assert_eq!(result["fileDialect"]["format"], "csv");
    assert_eq!(result["fileDialect"]["delimiter"], ";");
}

#[test]
fn test_should_return_none_for_inline_data() {
    assert_eq!(infer(json!({"data": [{"id": 1}]})), json!({}));
}

#[test]
fn test_should_serialize_without_unset_fields() {
    let path = write_temp_file("id,name\n1,alice\n", false, None, Some("csv")).unwrap();
    let dialect = infer(json!({"data": path}))["fileDialect"].clone();
    let dialect = dialect.as_object().unwrap();
    assert!(!dialect.contains_key("$schema"));
    assert!(dialect.values().all(|value| !value.is_null()));
}
