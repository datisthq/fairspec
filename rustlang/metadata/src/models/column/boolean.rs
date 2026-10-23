use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::base::{BooleanNullablePropertyType, StringOrInteger, ValueOrItem};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub r#type: BooleanNullablePropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Option<bool>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Option<bool>>>,
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
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub value: StringOrInteger,
    pub label: String,
}

define_column!(BooleanColumn, BooleanColumnProperty);
