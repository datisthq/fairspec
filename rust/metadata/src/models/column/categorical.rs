use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::base::ValueOrItem;
use super::integer::BaseIntegerColumnProperty;
use super::string::BaseStringColumnProperty;

define_literal!(CategoricalFormat, Categorical, "categorical");

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegerCategoricalColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseIntegerColumnProperty,
    pub format: CategoricalFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<IntegerCategory>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_order: Option<bool>,
}

pub type IntegerCategory = ValueOrItem<i64, IntegerCategoryItem>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct IntegerCategoryItem {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub value: i64,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringCategoricalColumnProperty {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(flatten)]
    pub base: BaseStringColumnProperty,
    pub format: CategoricalFormat,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<StringCategory>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_order: Option<bool>,
}

pub type StringCategory = ValueOrItem<String, StringCategoryItem>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct StringCategoryItem {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub value: String,
    pub label: String,
}

/// The base `type` decides the variant: `"string"` or `"integer"`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CategoricalColumnProperty {
    String(StringCategoricalColumnProperty),
    Integer(IntegerCategoricalColumnProperty),
}

impl Default for CategoricalColumnProperty {
    fn default() -> Self {
        CategoricalColumnProperty::String(StringCategoricalColumnProperty::default())
    }
}

define_column!(CategoricalColumn, CategoricalColumnProperty);
