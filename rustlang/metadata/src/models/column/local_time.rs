use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(LocalTimeFormat, LocalTime, "local-time");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalTimeColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: LocalTimeFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal_format: Option<String>,
}

define_column!(LocalTimeColumn, LocalTimeColumnProperty);
