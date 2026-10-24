use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::CreatorNameType;

pub type Creators = Vec<Creator>;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_type: Option<CreatorNameType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_identifiers: Option<Vec<CreatorNameIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliation: Option<Vec<CreatorAffiliation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorNameIdentifier {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub name_identifier: String,
    pub name_identifier_scheme: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorAffiliation {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliation_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affiliation_identifier_scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme_uri: Option<String>,
}
