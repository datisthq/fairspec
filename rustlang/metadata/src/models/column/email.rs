use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(EmailFormat, Email, "email");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EmailColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: EmailFormat,
}

define_column!(EmailColumn, EmailColumnProperty);
