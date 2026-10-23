use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_identifier_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
