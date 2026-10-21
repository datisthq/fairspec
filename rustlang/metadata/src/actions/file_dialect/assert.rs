use super::validate::validate_file_dialect;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::report::Report;

pub fn assert_file_dialect(source: &Descriptor) -> Result<FileDialect> {
    let result = validate_file_dialect(source, None)?;

    match result.file_dialect {
        Some(file_dialect) => Ok(file_dialect),
        None => Err(Error::Invalid {
            message: "Invalid dialect".to_string(),
            report: Some(Report {
                valid: result.valid,
                errors: result.errors,
            }),
        }),
    }
}
