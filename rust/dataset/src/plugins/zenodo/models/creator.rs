use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoCreatorIdentifier {
    pub identifier: String,
    pub scheme: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoCreator {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<ZenodoCreatorIdentifier>>,
}
