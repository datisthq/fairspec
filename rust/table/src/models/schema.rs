use fairspec_metadata::{Descriptor, TableSchema};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use super::column::PolarsColumn;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PolarsSchema {
    pub columns: Vec<PolarsColumn>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SchemaMapping {
    pub source: PolarsSchema,
    pub target: TableSchema,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableSchemaOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_types: Option<IndexMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_properties: Option<IndexMap<String, Descriptor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_char: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_char: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub true_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub false_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datetime_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_item_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InferTableSchemaOptions {
    #[serde(flatten)]
    pub table_schema_options: TableSchemaOptions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rows: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comma_decimal: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub month_first: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_strings: Option<bool>,
}
