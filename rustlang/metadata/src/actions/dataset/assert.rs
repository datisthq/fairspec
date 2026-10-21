use super::validate::validate_dataset_descriptor;
use crate::models::dataset::Dataset;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::report::Report;

pub fn assert_dataset(source: &Descriptor, basepath: Option<&str>) -> Result<Dataset> {
    let result = validate_dataset_descriptor(source, basepath)?;

    match result.dataset {
        Some(dataset) => Ok(dataset),
        None => Err(Error::Invalid {
            message: "Invalid Dataset".to_string(),
            report: Some(Report {
                valid: result.valid,
                errors: result.errors,
            }),
        }),
    }
}
