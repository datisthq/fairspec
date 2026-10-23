use fairspec_library::fairspec_metadata::{Report, Result};
use fairspec_library::validate_dataset;
use serde::{Deserialize, Serialize};

use crate::helpers::limit::{MAX_LIMIT, limit_report};
use crate::tools::_shared::schemas::check_range;
use crate::tools::_shared::source::DatasetInput;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDatasetInput {
    pub source: DatasetInput,
    #[serde(default)]
    pub max_errors: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateDatasetResult {
    pub report: Report,
    pub truncated: bool,
}

/// Validate a dataset including all its resources and foreign keys.
pub fn validate_dataset_tool(input: ValidateDatasetInput) -> Result<ValidateDatasetResult> {
    check_range("maxErrors", input.max_errors, 1, MAX_LIMIT)?;
    let report = validate_dataset(input.source, &Default::default())?;
    let (report, truncated) = limit_report(report, input.max_errors);
    Ok(ValidateDatasetResult { report, truncated })
}
