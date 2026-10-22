use fairspec_metadata::{FileDialect, TableSchema};
use serde::{Deserialize, Serialize};

use super::schema::{InferTableSchemaOptions, TableSchemaOptions};

pub type Table = polars::prelude::LazyFrame;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadTableOptions {
    #[serde(flatten)]
    pub infer_table_schema_options: InferTableSchemaOptions,
    #[serde(
        rename = "sample_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_bytes: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_bytes: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denormalized: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveTableOptions {
    #[serde(flatten)]
    pub table_schema_options: TableSchemaOptions,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_dialect: Option<FileDialect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_schema: Option<TableSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}
