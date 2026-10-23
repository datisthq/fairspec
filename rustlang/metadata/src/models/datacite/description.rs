use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::DescriptionType;

pub type Descriptions = Vec<DataciteDescription>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataciteDescription {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub description: String,
    pub description_type: DescriptionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
