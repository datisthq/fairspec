use serde_json::Value;

use crate::actions::json::inspect::{JsonIssue, combine_json_pointers};
use crate::models::descriptor::Descriptor;

pub fn inspect_json_schema(
    descriptor: &Descriptor,
    root_json_pointer: Option<&str>,
) -> Vec<JsonIssue> {
    let value = Value::Object(descriptor.clone());
    jsonschema::draft202012::meta::validator()
        .iter_errors(&value)
        .map(|error| JsonIssue {
            message: error.to_string(),
            json_pointer: combine_json_pointers(
                root_json_pointer.unwrap_or_default(),
                error.instance_path().as_str(),
            ),
        })
        .collect()
}
