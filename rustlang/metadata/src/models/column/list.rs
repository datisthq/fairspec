use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(ListFormat, List, "list");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: ListFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_type: Option<ListItemType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ListItemType {
    String,
    Integer,
    Number,
    Boolean,
    Date,
    Time,
    LocalTime,
    DateTime,
    LocalDateTime,
}

define_column!(ListColumn, ListColumnProperty);
