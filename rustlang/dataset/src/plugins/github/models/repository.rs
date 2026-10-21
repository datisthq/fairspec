use serde::{Deserialize, Serialize};

use super::file::GithubFile;
use super::license::GithubLicense;
use super::owner::GithubOwner;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubRepository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: GithubOwner,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub homepage: Option<String>,
    pub size: i64,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<String>,
    pub license: Option<GithubLicense>,
    pub default_branch: String,
    pub topics: Vec<String>,
    pub private: bool,
    pub archived: bool,
    pub html_url: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<GithubFile>>,
}
