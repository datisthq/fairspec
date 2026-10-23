use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::TitleType;

pub type Titles = Vec<Title>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_type: Option<TitleType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
