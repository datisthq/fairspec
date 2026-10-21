use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;
use crate::models::table_schema::TableSchemaValidationResult;

pub fn validate_table_schema(
    source: impl Into<PathOr<Descriptor>>,
    root_json_pointer: Option<&str>,
) -> Result<TableSchemaValidationResult> {
    let (descriptor, report) =
        validate_profiled_descriptor(source, ProfileType::TableSchema, root_json_pointer)?;

    let table_schema = match report.valid {
        true => Some(
            serde_json::from_value(descriptor.into())
                .map_err(|error| Error::Parse(error.to_string()))?,
        ),
        false => None,
    };

    Ok(TableSchemaValidationResult {
        valid: report.valid,
        errors: report.errors,
        table_schema,
    })
}
