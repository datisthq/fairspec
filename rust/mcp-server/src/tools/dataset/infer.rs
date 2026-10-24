use fairspec_library::fairspec_metadata::{Dataset, Result};
use fairspec_library::infer_dataset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct InferDatasetInput {
    pub dataset: Dataset,
}

#[derive(Debug, Clone, Serialize)]
pub struct InferDatasetResult {
    pub dataset: Dataset,
}

/// Infer metadata for all resources in a dataset.
pub fn infer_dataset_tool(input: InferDatasetInput) -> Result<InferDatasetResult> {
    Ok(InferDatasetResult {
        dataset: infer_dataset(&input.dataset, &Default::default())?,
    })
}
