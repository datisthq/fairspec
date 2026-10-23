use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(HexFormat, Hex, "hex");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HexColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: HexFormat,
}

define_column!(HexColumn, HexColumnProperty);
