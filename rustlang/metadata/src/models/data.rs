use serde::{Deserialize, Serialize};

use super::descriptor::Descriptor;
use super::path::Path;

pub type Data = serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceData {
    Path(ResourceDataPath),
    Value(ResourceDataValue),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceDataPath {
    Path(Path),
    Paths(Vec<Path>),
}

impl From<&str> for ResourceDataPath {
    fn from(path: &str) -> Self {
        ResourceDataPath::Path(path.to_string())
    }
}

impl From<String> for ResourceDataPath {
    fn from(path: String) -> Self {
        ResourceDataPath::Path(path)
    }
}

impl From<Vec<String>> for ResourceDataPath {
    fn from(paths: Vec<String>) -> Self {
        ResourceDataPath::Paths(paths)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceDataValue {
    Object(Descriptor),
    Objects(Vec<Descriptor>),
}
