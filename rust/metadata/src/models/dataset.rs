use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::datacite::datacite::Datacite;
use super::error::FairspecError;
use super::resource::Resource;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Dataset {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(flatten)]
    pub datacite: Datacite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderDatasetOptions {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConvertDatasetToOptions {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ConvertDatasetFromOptions {
    #[serde(flatten, default, skip_serializing_if = "Map::is_empty")]
    pub extra: Map<String, Value>,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DatasetValidationResult {
    pub valid: bool,
    pub errors: Vec<FairspecError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}
