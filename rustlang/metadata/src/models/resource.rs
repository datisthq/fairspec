use serde::{Deserialize, Serialize};

use super::data::Data;
use super::data_schema::DataSchema;
use super::datacite::datacite::Datacite;
use super::descriptor::Descriptor;
use super::file_dialect::file_dialect::FileDialect;
use super::integrity::Integrity;
use super::path::PathOr;
use super::table_schema::TableSchema;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(flatten)]
    pub datacite: Datacite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textual: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity: Option<Integrity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_dialect: Option<PathOr<FileDialect>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<PathOr<DataSchema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_schema: Option<PathOr<TableSchema>>,
    #[serde(
        rename = "unstable_customMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unstable_custom_metadata: Option<Descriptor>,
}
