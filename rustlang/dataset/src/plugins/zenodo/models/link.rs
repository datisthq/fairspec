use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoLink {
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    pub html: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit: Option<String>,
}
