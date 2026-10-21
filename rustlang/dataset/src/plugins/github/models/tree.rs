use serde::{Deserialize, Serialize};

use super::file::GithubFile;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GithubTree {
    pub tree: Vec<GithubFile>,
}
