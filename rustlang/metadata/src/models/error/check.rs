use serde::{Deserialize, Serialize};

define_literal!(TableCheckErrorType, TableCheck, "table/check");
define_literal!(CheckErrorType, Check, "check");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableCheckError {
    pub r#type: TableCheckErrorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub check_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckError {
    pub r#type: CheckErrorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub check_name: String,
    pub message: String,
}
