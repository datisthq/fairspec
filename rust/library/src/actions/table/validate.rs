use fairspec_metadata::{
    ExpectedResourceType, FairspecError, PathOr, Report, Resource, ResourceError,
    ResourceTypeError, Result, create_report, resolve_table_schema,
};
use fairspec_table::{LoadTableOptions, inspect_table, inspect_table_schema_patterns};

use super::load::load_table;
use crate::actions::file_dialect::infer::infer_file_dialect;
use crate::actions::table_schema::infer::infer_table_schema;
use crate::models::table::ValidateTableOptions;

pub fn validate_table(resource: &Resource, options: &ValidateTableOptions) -> Result<Report> {
    let mut resource = resource.clone();

    if resource.file_dialect.is_none() {
        resource.file_dialect =
            infer_file_dialect(&resource, &Default::default())?.map(PathOr::Value);
    }

    let no_infer = options.no_infer.unwrap_or(false);
    let max_errors = options.max_errors.unwrap_or(1000);
    let checks = options.checks.as_deref().unwrap_or_default();
    let load_table_options = &options.load_table_options;
    let infer_options = &load_table_options.infer_table_schema_options;

    let mut table_schema = resolve_table_schema(resource.table_schema.as_ref())?;
    if table_schema.is_none() && !no_infer {
        table_schema = infer_table_schema(&resource, infer_options)?;
    }

    if table_schema.is_none() && checks.is_empty() {
        return Ok(create_report(Vec::new(), None));
    }

    let pattern_errors = table_schema
        .as_ref()
        .map(inspect_table_schema_patterns)
        .unwrap_or_default();
    if !pattern_errors.is_empty() {
        return Ok(create_report(pattern_errors, Some(max_errors)));
    }

    let load_options = LoadTableOptions {
        denormalized: Some(true),
        ..load_table_options.clone()
    };
    let Some(table) = load_table(&resource, &load_options)? else {
        let error = ResourceTypeError {
            resource_name: None,
            expected_resource_type: ExpectedResourceType::Table,
            referencing_resource_name: None,
        };
        return Ok(create_report(
            vec![FairspecError::Resource(ResourceError::Type(error))],
            None,
        ));
    };

    let errors = inspect_table(
        table,
        table_schema.as_ref(),
        Some(checks),
        None,
        Some(max_errors),
        infer_options.concurrency,
    )?;
    Ok(create_report(
        errors.into_iter().map(FairspecError::Table).collect(),
        None,
    ))
}
