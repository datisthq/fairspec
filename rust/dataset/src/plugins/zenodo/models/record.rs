use serde::{Deserialize, Serialize};

use super::creator::ZenodoCreator;
use super::file::ZenodoFile;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoRecordLicense {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoRecordMetadata {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub creators: Vec<ZenodoCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<ZenodoRecordLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZenodoRecord {
    pub id: i64,
    pub metadata: ZenodoRecordMetadata,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<ZenodoFile>>,
}
