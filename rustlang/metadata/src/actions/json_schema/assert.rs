use super::inspect::inspect_json_schema;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::json_schema::JsonSchema;

pub fn assert_json_schema(descriptor: Descriptor) -> Result<JsonSchema> {
    let issues = inspect_json_schema(&descriptor, None);

    if !issues.is_empty() {
        let text = serde_json::to_string(&descriptor).unwrap_or_default();
        let preview: String = text.chars().take(100).collect();
        return Err(Error::Invalid {
            message: format!("JsonSchema \"{preview}\" is not valid"),
            report: None,
        });
    }

    Ok(descriptor)
}
