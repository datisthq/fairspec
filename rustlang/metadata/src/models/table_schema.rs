use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::column::base::ValueOrItem;
use super::column::column::ColumnProperty;
use super::error::FairspecError;
use super::foreign_key::ForeignKey;
use super::unique_key::UniqueKey;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSchema {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IndexMap<String, ColumnProperty>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<TableSchemaMissingValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_keys: Option<Vec<UniqueKey>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreign_keys: Option<Vec<ForeignKey>>,
}

pub type TableSchemaMissingValue = ValueOrItem<StringOrNumber, TableSchemaMissingValueItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSchemaMissingValueItem {
    pub value: StringOrNumber,
    pub label: String,
}

/// Python's `str | int | float`: an integer stays an integer on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Integer(i64),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderTableSchemaOptions {
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConvertTableSchemaToOptions {
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConvertTableSchemaFromOptions {
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TableSchemaValidationResult {
    pub valid: bool,
    pub errors: Vec<FairspecError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_schema: Option<TableSchema>,
}
