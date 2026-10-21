use serde::{Deserialize, Serialize};

use super::error::FairspecError;
use super::json_schema::JsonSchema;

pub type DataSchema = JsonSchema;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RenderDataSchemaOptions {
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DataSchemaValidationResult {
    pub valid: bool,
    pub errors: Vec<FairspecError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<DataSchema>,
}
