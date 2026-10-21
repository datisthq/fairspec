use serde::{Deserialize, Serialize};

use super::string::BaseStringColumnProperty;

define_literal!(UrlFormat, Url, "url");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UrlColumnProperty {
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: UrlFormat,
}

define_column!(UrlColumn, UrlColumnProperty);
