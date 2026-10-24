use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::{
    ColumnNames, CommentPrefix, CommentRows, Delimiter, HeaderJoin, HeaderRows, LineTerminator,
    NullSequence, QuoteChar,
};

define_literal!(CsvFormat, Csv, "csv");

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvFileDialect {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub format: CsvFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<Delimiter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_terminator: Option<LineTerminator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_char: Option<QuoteChar>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_sequence: Option<NullSequence>,
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
