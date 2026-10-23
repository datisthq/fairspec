use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(WktFormat, Wkt, "wkt");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WktColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: WktFormat,
}

define_column!(WktColumn, WktColumnProperty);
