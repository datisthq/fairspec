use serde::{Deserialize, Serialize};

use super::object::BaseObjectColumnProperty;

define_literal!(TopojsonFormat, Topojson, "topojson");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TopojsonColumnProperty {
    #[serde(flatten)]
    pub base: BaseObjectColumnProperty,
    pub format: TopojsonFormat,
}

define_column!(TopojsonColumn, TopojsonColumnProperty);
