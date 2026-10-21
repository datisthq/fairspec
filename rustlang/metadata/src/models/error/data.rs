use serde::{Deserialize, Serialize};

define_literal!(DataErrorType, Data, "data");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataError {
    pub r#type: DataErrorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub message: String,
    pub json_pointer: String,
}
