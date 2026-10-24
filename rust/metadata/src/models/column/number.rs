use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::base::{NumberNullablePropertyType, StringOrInteger, ValueOrItem};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseNumberColumnProperty {
    pub r#type: NumberNullablePropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Option<f64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Option<f64>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<NumberMissingValue>>,
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

pub type NumberMissingValue = ValueOrItem<StringOrInteger, NumberMissingValueItem>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumberMissingValueItem {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub value: StringOrInteger,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NumberColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(flatten)]
    pub base: BaseNumberColumnProperty,
}

define_column!(NumberColumn, NumberColumnProperty);
