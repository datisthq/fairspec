use serde_json::json;

use crate::tools::data_schema::infer::{InferDataSchemaInput, infer_data_schema_tool};

#[test]
fn test_should_infer_schema_from_inline_data() {
    let input: InferDataSchemaInput =
        serde_json::from_value(json!({"resource": {"data": {"name": "alice", "age": 25}}}))
            .unwrap();
    let schema = infer_data_schema_tool(input).unwrap().data_schema.unwrap();
    assert_eq!(schema["type"], "object");
    let mut names: Vec<&String> = schema["properties"].as_object().unwrap().keys().collect();
    names.sort();
    assert_eq!(names, ["age", "name"]);
}

#[test]
fn test_should_return_none_for_resource_without_data() {
    let input: InferDataSchemaInput = serde_json::from_value(json!({"resource": {}})).unwrap();
    let result = infer_data_schema_tool(input).unwrap();
    assert_eq!(result.data_schema, None);
    assert_eq!(serde_json::to_value(result).unwrap(), json!({}));
}
