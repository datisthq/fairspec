use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(DateFormat, Date, "date");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: DateFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal_format: Option<String>,
}

define_column!(DateColumn, DateColumnProperty);
