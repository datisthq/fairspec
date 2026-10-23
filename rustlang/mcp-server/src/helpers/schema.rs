use std::sync::{Arc, LazyLock};

use rmcp::model::{JsonObject, Tool};
use serde_json::Value;

/// The tools Python's server lists, with every schema's `$defs` as a list of names.
const TOOLS: &str = include_str!("../../schemas/tools.json");
/// The definitions those names point to, shared by every schema.
const DEFS: &str = include_str!("../../schemas/defs.json");

/// The tools with Python's exact names, descriptions and schemas.
pub static TOOL_DEFINITIONS: LazyLock<Vec<Tool>> = LazyLock::new(create_tool_definitions);

fn create_tool_definitions() -> Vec<Tool> {
    let tools: Vec<JsonObject> = serde_json::from_str(TOOLS).expect("bundled tools are JSON");
    let defs: JsonObject = serde_json::from_str(DEFS).expect("bundled defs are JSON");

    tools
        .into_iter()
        .map(|tool| {
            let get_schema = |key: &str| match tool.get(key) {
                Some(Value::Object(schema)) => Some(Arc::new(resolve_defs(schema, &defs))),
                _ => None,
            };
            let mut definition = Tool::default();
            definition.name = tool["name"].as_str().unwrap_or_default().to_string().into();
            definition.description = tool["description"]
                .as_str()
                .map(|text| text.to_string().into());
            definition.input_schema = get_schema("inputSchema").unwrap_or_default();
            definition.output_schema = get_schema("outputSchema");
            definition
        })
        .collect()
}

/// Puts each listed definition in place of its name, keeping the keys' order.
fn resolve_defs(schema: &JsonObject, defs: &JsonObject) -> JsonObject {
    schema
        .iter()
        .map(|(key, value)| match (key.as_str(), value) {
            ("$defs", Value::Array(names)) => {
                let resolved = names
                    .iter()
                    .filter_map(Value::as_str)
                    .map(|name| (name.to_string(), defs[name].clone()))
                    .collect();
                (key.clone(), Value::Object(resolved))
            }
            _ => (key.clone(), value.clone()),
        })
        .collect()
}
