use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(UrlFormat, Url, "url");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UrlColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: UrlFormat,
}

define_column!(UrlColumn, UrlColumnProperty);
