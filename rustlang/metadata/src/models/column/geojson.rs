use serde::{Deserialize, Serialize};

use super::object::BaseObjectColumnProperty;

define_literal!(GeojsonFormat, Geojson, "geojson");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GeojsonColumnProperty {
    #[serde(flatten)]
    pub base: BaseObjectColumnProperty,
    pub format: GeojsonFormat,
}

define_column!(GeojsonColumn, GeojsonColumnProperty);
