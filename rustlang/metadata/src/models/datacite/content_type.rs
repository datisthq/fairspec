use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::ContentTypeGeneral;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentTypes {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub resource_type_general: ContentTypeGeneral,
}
