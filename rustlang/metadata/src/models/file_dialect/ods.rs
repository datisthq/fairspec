use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::{
    ColumnNames, CommentPrefix, CommentRows, HeaderJoin, HeaderRows, SheetName, SheetNumber,
};

define_literal!(OdsFormat, Ods, "ods");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OdsFileDialect {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub format: OdsFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_name: Option<SheetName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet_number: Option<SheetNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_rows: Option<HeaderRows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_join: Option<HeaderJoin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_rows: Option<CommentRows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_prefix: Option<CommentPrefix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_names: Option<ColumnNames>,
}
