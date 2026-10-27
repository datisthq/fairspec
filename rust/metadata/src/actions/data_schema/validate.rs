use super::inspect::inspect_data_schema;
use crate::actions::descriptor::load::load_descriptor_source;
use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::models::data_schema::DataSchemaValidationResult;
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::{MetadataError, MetadataErrorType};
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;

pub fn validate_data_schema(
    source: impl Into<PathOr<Descriptor>>,
    root_json_pointer: Option<&str>,
) -> Result<DataSchemaValidationResult> {
    let descriptor = load_descriptor_source(source)?;

    let schema_errors: Vec<FairspecError> = inspect_data_schema(&descriptor, root_json_pointer)
        .into_iter()
        .map(|issue| {
            FairspecError::Metadata(MetadataError {
                r#type: MetadataErrorType::default(),
                resource_name: None,
                message: issue.message,
                json_pointer: issue.json_pointer,
            })
        })
        .collect();
    let is_schema_error = matches!(
        schema_errors.first(),
        Some(FairspecError::Metadata(error)) if error.json_pointer.ends_with("/$schema")
    );
    if is_schema_error {
        return Ok(DataSchemaValidationResult {
            valid: false,
            errors: schema_errors,
            data_schema: None,
        });
    }

    let (descriptor, report) = validate_profiled_descriptor(
        PathOr::Value(descriptor),
        ProfileType::DataSchema,
        root_json_pointer,
    )?;
    let mut errors = report.errors;
    errors.extend(schema_errors);
    let valid = errors.is_empty();

    Ok(DataSchemaValidationResult {
        data_schema: valid.then_some(descriptor),
        valid,
        errors,
    })
}
