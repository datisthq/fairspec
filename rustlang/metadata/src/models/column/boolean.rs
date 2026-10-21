use serde::{Deserialize, Serialize};

use super::base::{BooleanNullablePropertyType, StringOrInteger, ValueOrItem};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanColumnProperty {
    pub r#type: BooleanNullablePropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<BooleanMissingValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub true_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub false_values: Option<Vec<String>>,
}

pub type BooleanMissingValue = ValueOrItem<StringOrInteger, StringIntMissingValue>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StringIntMissingValue {
    pub value: StringOrInteger,
    pub label: String,
}

define_column!(BooleanColumn, BooleanColumnProperty);
