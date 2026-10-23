use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::common::{ContributorType, CreatorNameType};
use super::creator::{CreatorAffiliation, CreatorNameIdentifier};

pub type Contributors = Vec<Contributor>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
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
    pub contributor_type: ContributorType,
}
