use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::base::{ObjectNullablePropertyType, ValueOrItem};
use crate::models::descriptor::Descriptor;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseObjectColumnProperty {
    pub r#type: ObjectNullablePropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<Descriptor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#const: Option<Descriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<Descriptor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Descriptor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_values: Option<Vec<ObjectMissingValue>>,
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
    pub properties: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_properties: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property_names: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_required: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependent_schemas: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Value>,
}

pub type ObjectMissingValue = ValueOrItem<String, ObjectMissingValueItem>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ObjectMissingValueItem {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectColumnProperty {
    #[serde(flatten)]
    pub base: BaseObjectColumnProperty,
}

define_column!(ObjectColumn, ObjectColumnProperty);
