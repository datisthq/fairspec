use serde::{Deserialize, Serialize};

use super::base::{IntegerNullablePropertyType, StringOrInteger, ValueOrItem};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseIntegerColumnProperty {
    pub r#type: IntegerNullablePropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<IntegerMissingValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_char: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_text: Option<bool>,
}

pub type IntegerMissingValue = ValueOrItem<StringOrInteger, IntegerMissingValueItem>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegerMissingValueItem {
    pub value: StringOrInteger,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IntegerColumnProperty {
    #[serde(flatten)]
    pub base: BaseIntegerColumnProperty,
}

define_column!(IntegerColumn, IntegerColumnProperty);
