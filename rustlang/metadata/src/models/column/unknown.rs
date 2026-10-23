use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::base::ValueOrItem;

define_literal!(NullPropertyType, Null, "null");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnknownColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<NullPropertyType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<UnknownMissingValue>>,
}

pub type UnknownMissingValue = ValueOrItem<String, UnknownMissingValueItem>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UnknownMissingValueItem {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub value: String,
    pub label: String,
}

define_column!(UnknownColumn, UnknownColumnProperty);
