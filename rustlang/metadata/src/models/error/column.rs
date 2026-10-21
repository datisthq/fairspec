use serde::{Deserialize, Serialize};

use crate::models::column::column::ColumnType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ColumnError {
    #[serde(rename = "column/missing")]
    Missing(ColumnMissingError),
    #[serde(rename = "column/type")]
    Type(ColumnTypeError),
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnMissingError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnTypeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub expected_column_type: ColumnType,
    pub actual_column_type: ColumnType,
}
