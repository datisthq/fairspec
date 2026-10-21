use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(HexFormat, Hex, "hex");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HexColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: HexFormat,
}

define_column!(HexColumn, HexColumnProperty);
