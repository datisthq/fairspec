use serde::{Deserialize, Serialize};

define_literal!(MetadataErrorType, Metadata, "metadata");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataError {
    pub r#type: MetadataErrorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub message: String,
    pub json_pointer: String,
}
