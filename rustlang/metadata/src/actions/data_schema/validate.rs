use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::models::data_schema::DataSchemaValidationResult;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;

pub fn validate_data_schema(
    source: impl Into<PathOr<Descriptor>>,
    root_json_pointer: Option<&str>,
) -> Result<DataSchemaValidationResult> {
    let (descriptor, report) =
        validate_profiled_descriptor(source, ProfileType::DataSchema, root_json_pointer)?;

    Ok(DataSchemaValidationResult {
        data_schema: report.valid.then_some(descriptor),
        valid: report.valid,
        errors: report.errors,
    })
}
