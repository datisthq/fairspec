use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(LocalDateTimeFormat, LocalDateTime, "local-date-time");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalDateTimeColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: LocalDateTimeFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal_format: Option<String>,
}

define_column!(LocalDateTimeColumn, LocalDateTimeColumnProperty);
