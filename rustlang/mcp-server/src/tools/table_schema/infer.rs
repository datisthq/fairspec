use fairspec_library::fairspec_metadata::{Resource, Result, TableSchema};
use fairspec_library::fairspec_table::InferTableSchemaOptions;
use fairspec_library::infer_table_schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct InferTableSchemaInput {
    pub resource: Resource,
    #[serde(default)]
    pub options: Option<InferTableSchemaOptions>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferTableSchemaResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_schema: Option<TableSchema>,
}

/// Infer a table schema from table resource data.
pub fn infer_table_schema_tool(input: InferTableSchemaInput) -> Result<InferTableSchemaResult> {
    let options = input.options.unwrap_or_default();
    Ok(InferTableSchemaResult {
        table_schema: infer_table_schema(&input.resource, &options)?,
    })
}
