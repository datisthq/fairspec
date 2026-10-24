use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubLicense {
    pub key: String,
    pub name: String,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
}
