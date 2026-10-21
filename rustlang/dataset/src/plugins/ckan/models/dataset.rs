use serde::{Deserialize, Serialize};

use super::organization::CkanOrganization;
use super::resource::{CkanResource, NewCkanResource};
use super::tag::CkanTag;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CkanDataset {
    pub resources: Vec<CkanResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<CkanOrganization>,
    pub tags: Vec<CkanTag>,
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_modified: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NewCkanDataset {
    pub resources: Vec<NewCkanResource>,
    pub tags: Vec<CkanTag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_modified: Option<String>,
}
