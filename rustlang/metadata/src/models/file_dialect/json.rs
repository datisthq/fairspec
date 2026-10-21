use serde::{Deserialize, Serialize};

use super::common::{
    ColumnNames, CommentPrefix, CommentRows, HeaderJoin, HeaderRows, JsonPointer, RowType,
};

define_literal!(JsonFormat, Json, "json");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonFileDialect {
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub format: JsonFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_pointer: Option<JsonPointer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_type: Option<RowType>,
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
