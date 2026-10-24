use serde::{Deserialize, Serialize};

use super::creator::ZenodoCreator;

#[expect(dead_code, reason = "unused by the Python twin too, ported for parity")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoRelatedIdentifier {
    pub identifier: String,
    pub relation: String,
    pub scheme: String,
}

#[expect(dead_code, reason = "unused by the Python twin too, ported for parity")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoCommunity {
    pub identifier: String,
}

#[expect(dead_code, reason = "unused by the Python twin too, ported for parity")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoMetadata {
    pub title: String,
    pub description: String,
    pub upload_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    pub creators: Vec<ZenodoCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_right: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_identifiers: Option<Vec<ZenodoRelatedIdentifier>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communities: Option<Vec<ZenodoCommunity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
