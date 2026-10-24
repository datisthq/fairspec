use fairspec_library::fairspec_dataset::InferFileDialectOptions;
use fairspec_library::fairspec_metadata::{FileDialect, Resource, Result};
use fairspec_library::infer_file_dialect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct InferFileDialectInput {
    pub resource: Resource,
    #[serde(default)]
    pub options: Option<InferFileDialectToolOptions>,
}

/// Python's `InferFileDialectOptions` on the wire, snake_case key included.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InferFileDialectToolOptions {
    #[serde(default)]
    pub sample_bytes: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InferFileDialectResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_dialect: Option<FileDialect>,
}

/// Infer the dialect (format and format-specific properties) from a resource.
pub fn infer_file_dialect_tool(input: InferFileDialectInput) -> Result<InferFileDialectResult> {
    let options = InferFileDialectOptions {
        sample_bytes: input.options.unwrap_or_default().sample_bytes,
    };
    Ok(InferFileDialectResult {
        file_dialect: infer_file_dialect(&input.resource, &options)?,
    })
}
