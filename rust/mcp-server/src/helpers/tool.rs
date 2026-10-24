use fairspec_library::fairspec_metadata::{Error, Result};
use rmcp::model::{CallToolResult, ContentBlock};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::limit::MAX_RESPONSE_CHARS;

/// Passes a tool's failure to the client with its message, caps the response size, and
/// sends compact JSON text next to the structured content.
pub fn guard_tool<T: Serialize>(name: &str, result: Result<T>) -> CallToolResult {
    let value = match result.and_then(|result| {
        serde_json::to_value(result).map_err(|error| Error::Parse(error.to_string()))
    }) {
        Ok(value) => value,
        Err(error) => return create_error_result(name, &error.to_string()),
    };

    let text = value.to_string();
    if text.chars().count() > MAX_RESPONSE_CHARS {
        let message =
            format!("The response exceeds {MAX_RESPONSE_CHARS} characters; narrow the input");
        return create_error_result(name, &message);
    }

    let mut result = CallToolResult::success(vec![ContentBlock::text(text)]);
    result.structured_content = Some(value);
    result
}

/// The arguments a tool takes, read from the call's JSON object.
pub fn parse_tool_input<T: DeserializeOwned>(arguments: Option<Value>) -> Result<T> {
    let arguments = arguments.unwrap_or_else(|| Value::Object(Default::default()));
    serde_json::from_value(arguments).map_err(|error| Error::Parse(error.to_string()))
}

pub fn create_error_result(name: &str, message: &str) -> CallToolResult {
    let text = format!("Error executing tool {name}: {message}");
    CallToolResult::error(vec![ContentBlock::text(text)])
}
