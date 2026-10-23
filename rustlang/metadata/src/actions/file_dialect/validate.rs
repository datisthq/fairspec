use serde::{Deserialize, Serialize};

use crate::actions::descriptor::inspect::inspect_model;
use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::exception::Result;
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FileDialectValidationResult {
    pub valid: bool,
    pub errors: Vec<FairspecError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_dialect: Option<FileDialect>,
}

pub fn validate_file_dialect(
    source: impl Into<PathOr<Descriptor>>,
    root_json_pointer: Option<&str>,
) -> Result<FileDialectValidationResult> {
    let (descriptor, mut report) =
        validate_profiled_descriptor(source, ProfileType::FileDialect, root_json_pointer)?;

    let file_dialect = match report.valid {
        true => {
            let (file_dialect, errors) = inspect_model(&descriptor, root_json_pointer);
            report.valid = errors.is_empty();
            report.errors.extend(errors);
            file_dialect
        }
        false => None,
    };

    Ok(FileDialectValidationResult {
        valid: report.valid,
        errors: report.errors,
        file_dialect,
    })
}
