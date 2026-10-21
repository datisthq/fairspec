use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(TimeFormat, Time, "time");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: TimeFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal_format: Option<String>,
}

define_column!(TimeColumn, TimeColumnProperty);
