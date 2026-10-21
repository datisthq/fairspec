use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(WkbFormat, Wkb, "wkb");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WkbColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: WkbFormat,
}

define_column!(WkbColumn, WkbColumnProperty);
