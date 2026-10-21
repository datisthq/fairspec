use serde::{Deserialize, Serialize};

use super::common::TitleType;

pub type Titles = Vec<Title>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_type: Option<TitleType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
