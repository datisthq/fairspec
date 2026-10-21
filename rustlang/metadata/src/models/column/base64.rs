use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(Base64Format, Base64, "base64");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Base64ColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: Base64Format,
}

define_column!(Base64Column, Base64ColumnProperty);
