use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub type AlternateIdentifiers = Vec<AlternateIdentifier>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternateIdentifier {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub alternate_identifier: String,
    pub alternate_identifier_type: String,
}
