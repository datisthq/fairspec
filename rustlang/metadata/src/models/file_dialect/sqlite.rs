use serde::{Deserialize, Serialize};

use super::common::TableName;

define_literal!(SqliteFormat, Sqlite, "sqlite");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqliteFileDialect {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub format: SqliteFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<TableName>,
}
