use super::validate::validate_data_schema;
use crate::models::data_schema::DataSchema;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::report::Report;

pub fn assert_data_schema(source: &Descriptor) -> Result<DataSchema> {
    let result = validate_data_schema(source, None)?;

    match result.data_schema {
        Some(data_schema) => Ok(data_schema),
        None => Err(Error::Invalid {
            message: "Invalid Data Schema".to_string(),
            report: Some(Report {
                valid: result.valid,
                errors: result.errors,
            }),
        }),
    }
}
