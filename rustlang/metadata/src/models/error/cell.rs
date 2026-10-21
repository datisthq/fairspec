use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::models::column::column::ColumnType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CellError {
    #[serde(rename = "cell/type")]
    Type(CellTypeError),
    #[serde(rename = "cell/missing")]
    Missing(CellMissingError),
    #[serde(rename = "cell/minimum")]
    Minimum(CellMinimumError),
    #[serde(rename = "cell/maximum")]
    Maximum(CellMaximumError),
    #[serde(rename = "cell/exclusiveMinimum")]
    ExclusiveMinimum(CellExclusiveMinimumError),
    #[serde(rename = "cell/exclusiveMaximum")]
    ExclusiveMaximum(CellExclusiveMaximumError),
    #[serde(rename = "cell/multipleOf")]
    MultipleOf(CellMultipleOfError),
    #[serde(rename = "cell/minLength")]
    MinLength(CellMinLengthError),
    #[serde(rename = "cell/maxLength")]
    MaxLength(CellMaxLengthError),
    #[serde(rename = "cell/minItems")]
    MinItems(CellMinItemsError),
    #[serde(rename = "cell/maxItems")]
    MaxItems(CellMaxItemsError),
    #[serde(rename = "cell/pattern")]
    Pattern(CellPatternError),
    #[serde(rename = "cell/unique")]
    Unique(CellUniqueError),
    #[serde(rename = "cell/const")]
    Const(CellConstError),
    #[serde(rename = "cell/enum")]
    Enum(CellEnumError),
    #[serde(rename = "cell/json")]
    Json(CellJsonError),
    #[serde(rename = "cell/check")]
    Check(CellCheckError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellTypeError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub column_type: ColumnType,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMissingError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMinimumError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub minimum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMaximumError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub maximum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellExclusiveMinimumError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub minimum: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellExclusiveMaximumError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub maximum: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMultipleOfError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub multiple_of: Number,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMinLengthError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub min_length: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMaxLengthError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub max_length: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMinItemsError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub min_items: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellMaxItemsError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub max_items: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellPatternError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellUniqueError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellConstError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub r#const: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellEnumError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub r#enum: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellJsonError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub message: String,
    pub json_pointer: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellCheckError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    pub column_name: String,
    pub row_number: usize,
    pub cell: String,
    pub check_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
