use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type RightsList = Vec<Rights>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rights {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights_identifier_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
