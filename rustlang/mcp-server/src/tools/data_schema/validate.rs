use fairspec_library::fairspec_metadata::{
    Descriptor, PathOr, Report, Result, validate_data_schema,
};
use serde::{Deserialize, Serialize};

use crate::helpers::limit::{MAX_LIMIT, limit_report};
use crate::tools::_shared::schemas::check_range;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDataSchemaInput {
    pub source: PathOr<Descriptor>,
    #[serde(default)]
    pub max_errors: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateDataSchemaResult {
    pub report: Report,
    pub truncated: bool,
}

/// Validate a data schema against the Fairspec Data Schema specification.
pub fn validate_data_schema_tool(
    input: ValidateDataSchemaInput,
) -> Result<ValidateDataSchemaResult> {
    check_range("maxErrors", input.max_errors, 1, MAX_LIMIT)?;
    let result = validate_data_schema(input.source, None)?;
    let report = Report {
        valid: result.valid,
        errors: result.errors,
    };
    let (report, truncated) = limit_report(report, input.max_errors);
    Ok(ValidateDataSchemaResult { report, truncated })
}
