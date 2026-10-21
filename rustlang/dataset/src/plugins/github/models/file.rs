use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubFile {
    pub path: String,
    pub mode: String,
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    pub sha: String,
    pub url: String,
}
