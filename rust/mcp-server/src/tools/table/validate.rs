use fairspec_library::fairspec_metadata::resolve_table_schema;
use fairspec_library::fairspec_metadata::{PathOr, Report, Resource, Result};
use fairspec_library::{ValidateTableOptions, infer_table_schema, validate_table};
use serde::{Deserialize, Serialize};

use crate::helpers::limit::{DEFAULT_LIMIT, MAX_LIMIT, limit_report};
use crate::tools::_shared::schemas::check_range;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTableInput {
    pub resource: Resource,
    #[serde(default)]
    pub options: Option<ValidateTableOptions>,
    #[serde(default)]
    pub max_errors: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidateTableResult {
    pub report: Report,
    pub truncated: bool,
}

/// Validate a table resource against its schema.
pub fn validate_table_tool(input: ValidateTableInput) -> Result<ValidateTableResult> {
    check_range("maxErrors", input.max_errors, 1, MAX_LIMIT)?;
    let mut options = input.options.unwrap_or_default();
    let max_errors = input
        .max_errors
        .or(options.max_errors.filter(|value| *value != 0))
        .unwrap_or(DEFAULT_LIMIT)
        .min(MAX_LIMIT);

    let mut table_schema = resolve_table_schema(input.resource.table_schema.as_ref())?;
    if table_schema.is_none() && options.no_infer != Some(true) {
        let infer_options = &options.load_table_options.infer_table_schema_options;
        table_schema = infer_table_schema(&input.resource, infer_options)?;
    }

    // The library splits maxErrors across row key checks by the column count, so ask
    // for one error more per column, or a single bad key would never show truncation
    let column_count = table_schema
        .as_ref()
        .and_then(|schema| schema.properties.as_ref())
        .map_or(0, |properties| properties.len())
        .max(1);
    options.max_errors = Some((max_errors + 1) * column_count);

    let resource = Resource {
        table_schema: table_schema.map(PathOr::Value),
        ..input.resource
    };
    let report = validate_table(&resource, &options)?;
    let (report, truncated) = limit_report(report, Some(max_errors));
    Ok(ValidateTableResult { report, truncated })
}
