use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::DateType;

pub type DateValue = String;
pub type Dates = Vec<DataciteDate>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataciteDate {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub date: DateValue,
    pub date_type: DateType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_information: Option<String>,
}
