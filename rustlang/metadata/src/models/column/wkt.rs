use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(WktFormat, Wkt, "wkt");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WktColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: WktFormat,
}

define_column!(WktColumn, WktColumnProperty);
