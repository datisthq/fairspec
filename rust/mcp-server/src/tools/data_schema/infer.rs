use fairspec_library::fairspec_metadata::{DataSchema, Resource, Result};
use fairspec_library::infer_data_schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct InferDataSchemaInput {
    pub resource: Resource,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferDataSchemaResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_schema: Option<DataSchema>,
}

/// Infer a data schema (JSON Schema) from data.
pub fn infer_data_schema_tool(input: InferDataSchemaInput) -> Result<InferDataSchemaResult> {
    Ok(InferDataSchemaResult {
        data_schema: infer_data_schema(&input.resource)?,
    })
}
