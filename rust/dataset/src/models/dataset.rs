use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SaveDatasetOptions {
    pub target: String,
    pub with_remote: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SaveDatasetResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
