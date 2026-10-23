use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::object::BaseObjectColumnProperty;

define_literal!(GeojsonFormat, Geojson, "geojson");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeojsonColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseObjectColumnProperty,
    pub format: GeojsonFormat,
}

define_column!(GeojsonColumn, GeojsonColumnProperty);
