use serde::{Deserialize, Serialize};

use super::common::DescriptionType;

pub type Descriptions = Vec<DataciteDescription>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataciteDescription {
    pub description: String,
    pub description_type: DescriptionType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
