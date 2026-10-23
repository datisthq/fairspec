use fairspec_library::fairspec_dataset::write_temp_file;
use serde_json::{Value, json};

use crate::tools::table_schema::infer::{InferTableSchemaInput, infer_table_schema_tool};

fn infer(options: Option<Value>) -> Value {
    let path = write_temp_file("id,name\n1,alice\n2,bob\n", false, None, Some("csv")).unwrap();
    let mut input = json!({"resource": {"data": path}});
    if let Some(options) = options {
        input["options"] = options;
    }
    let input: InferTableSchemaInput = serde_json::from_value(input).unwrap();
    serde_json::to_value(infer_table_schema_tool(input).unwrap()).unwrap()
}

#[test]
fn test_should_infer_schema_from_csv() {
    assert_eq!(
        infer(None)["tableSchema"]["properties"],
        json!({"id": {"type": "integer"}, "name": {"type": "string"}})
    );
}

#[test]
fn test_should_forward_options() {
    let result = infer(Some(json!({"columnTypes": {"id": "string"}})));
    assert_eq!(
        result["tableSchema"]["properties"]["id"],
        json!({"type": "string"})
    );
}

#[test]
fn test_should_forward_column_properties() {
    let result = infer(Some(
        json!({"columnProperties": {"id": {"type": "string"}, "name": {"title": "Name"}}}),
    ));
    assert_eq!(
        result["tableSchema"]["properties"]["id"],
        json!({"type": "string"})
    );
    assert_eq!(
        result["tableSchema"]["properties"]["name"],
        json!({"type": "string", "title": "Name"})
    );
}
