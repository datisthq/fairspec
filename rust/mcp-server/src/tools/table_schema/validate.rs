use fairspec_library::fairspec_metadata::{
    Descriptor, PathOr, Report, Result, validate_table_schema,
};
use serde::{Deserialize, Serialize};

use crate::helpers::limit::{MAX_LIMIT, limit_report};
use crate::tools::_shared::schemas::check_range;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTableSchemaInput {
    pub source: PathOr<Descriptor>,
    #[serde(default)]
    pub max_errors: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateTableSchemaResult {
    pub report: Report,
    pub truncated: bool,
}

/// Validate a table schema against the Fairspec Table Schema specification.
pub fn validate_table_schema_tool(
    input: ValidateTableSchemaInput,
) -> Result<ValidateTableSchemaResult> {
    check_range("maxErrors", input.max_errors, 1, MAX_LIMIT)?;
    let result = validate_table_schema(input.source, None)?;
    let report = Report {
        valid: result.valid,
        errors: result.errors,
    };
    let (report, truncated) = limit_report(report, input.max_errors);
    Ok(ValidateTableSchemaResult { report, truncated })
}
