use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::base::{ArrayNullablePropertyType, ValueOrItem};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrayColumnProperty {
    pub r#type: ArrayNullablePropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<Vec<Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Vec<Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<ArrayMissingValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_of: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub one_of: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub then: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_items: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_items: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_contains: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_contains: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
}

pub type ArrayMissingValue = ValueOrItem<String, ArrayMissingValueItem>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ArrayMissingValueItem {
    pub value: String,
    pub label: String,
}

define_column!(ArrayColumn, ArrayColumnProperty);
