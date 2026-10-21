use serde::{Deserialize, Serialize};

use super::common::DateType;

pub type DateValue = String;
pub type Dates = Vec<DataciteDate>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataciteDate {
    pub date: DateValue,
    pub date_type: DateType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_information: Option<String>,
}
