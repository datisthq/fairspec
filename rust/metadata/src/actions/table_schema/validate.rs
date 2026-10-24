use crate::actions::descriptor::inspect::inspect_model;
use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;
use crate::models::table_schema::{TableSchema, TableSchemaValidationResult};

use super::inspect::inspect_table_schema;

pub fn validate_table_schema(
    source: impl Into<PathOr<Descriptor>>,
    root_json_pointer: Option<&str>,
) -> Result<TableSchemaValidationResult> {
    let (descriptor, mut report) =
        validate_profiled_descriptor(source, ProfileType::TableSchema, root_json_pointer)?;

    let table_schema = match report.valid {
        true => {
            let (table_schema, errors) =
                inspect_model::<TableSchema>(&descriptor, root_json_pointer);
            report.errors.extend(errors);
            if let Some(table_schema) = &table_schema {
                report
                    .errors
                    .extend(inspect_table_schema(table_schema, root_json_pointer));
            }
            report.valid = report.errors.is_empty();
            table_schema.filter(|_| report.valid)
        }
        false => None,
    };

    Ok(TableSchemaValidationResult {
        valid: report.valid,
        errors: report.errors,
        table_schema,
    })
}
