use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::string::BaseStringColumnProperty;

define_literal!(DecimalFormat, Decimal, "decimal");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecimalColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: DecimalFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decimal_char: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_char: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_text: Option<bool>,
}

define_column!(DecimalColumn, DecimalColumnProperty);
