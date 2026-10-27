use crate::actions::descriptor::inspect::inspect_model;
use crate::actions::descriptor::load::load_descriptor_source;
use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::actions::frictionless::convert::{FrictionlessKind, prepare_frictionless_descriptor};
use crate::actions::frictionless::mapping::map_frictionless_errors;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;
use crate::models::table_schema::{TableSchema, TableSchemaValidationResult};

use super::inspect::{inspect_table_schema, inspect_table_schema_nulls};

pub fn validate_table_schema(
    source: impl Into<PathOr<Descriptor>>,
    root_json_pointer: Option<&str>,
) -> Result<TableSchemaValidationResult> {
    let loaded = load_descriptor_source(source)?;
    let (prepared, mapping, errors) =
        prepare_frictionless_descriptor(loaded, FrictionlessKind::TableSchema, root_json_pointer);
    let Some(prepared) = prepared else {
        return Ok(TableSchemaValidationResult {
            valid: false,
            errors,
            table_schema: None,
        });
    };

    let (descriptor, mut report) =
        validate_profiled_descriptor(prepared, ProfileType::TableSchema, root_json_pointer)?;

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
            report
                .errors
                .extend(inspect_table_schema_nulls(&descriptor, root_json_pointer));
            report.valid = report.errors.is_empty();
            table_schema.filter(|_| report.valid)
        }
        false => None,
    };

    if let Some(mapping) = &mapping {
        report.errors = map_frictionless_errors(report.errors, mapping);
    }

    Ok(TableSchemaValidationResult {
        valid: report.valid,
        errors: report.errors,
        table_schema,
    })
}
