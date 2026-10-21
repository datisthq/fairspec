use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubOwner {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    pub html_url: String,
    pub r#type: GithubOwnerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GithubOwnerType {
    User,
    Organization,
}
