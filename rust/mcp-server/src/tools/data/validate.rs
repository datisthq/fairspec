use fairspec_library::fairspec_metadata::{Report, Resource, Result};
use fairspec_library::validate_data;
use serde::{Deserialize, Serialize};

use crate::helpers::limit::{MAX_LIMIT, limit_report};
use crate::tools::_shared::schemas::check_range;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDataInput {
    pub resource: Resource,
    #[serde(default)]
    pub max_errors: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateDataResult {
    pub report: Report,
    pub truncated: bool,
}

/// Validate data against a data schema (JSON Schema).
pub fn validate_data_tool(input: ValidateDataInput) -> Result<ValidateDataResult> {
    check_range("maxErrors", input.max_errors, 1, MAX_LIMIT)?;
    let (report, truncated) = limit_report(validate_data(&input.resource)?, input.max_errors);
    Ok(ValidateDataResult { report, truncated })
}
