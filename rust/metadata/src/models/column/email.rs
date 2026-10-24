use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(EmailFormat, Email, "email");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EmailColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: EmailFormat,
}

define_column!(EmailColumn, EmailColumnProperty);
