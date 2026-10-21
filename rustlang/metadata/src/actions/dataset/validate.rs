use crate::actions::data_schema::validate::validate_data_schema;
use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::actions::file_dialect::validate::validate_file_dialect;
use crate::actions::table_schema::validate::validate_table_schema;
use crate::models::dataset::{Dataset, DatasetValidationResult};
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;

use super::normalize::normalize_dataset;

pub fn validate_dataset_descriptor(
    source: impl Into<PathOr<Descriptor>>,
    basepath: Option<&str>,
) -> Result<DatasetValidationResult> {
    let (descriptor, mut report) =
        validate_profiled_descriptor(source, ProfileType::Dataset, None)?;

    if !report.valid {
        return Ok(DatasetValidationResult {
            valid: report.valid,
            errors: report.errors,
            dataset: None,
        });
    }

    let dataset: Dataset = serde_json::from_value(descriptor.into())
        .map_err(|error| Error::Parse(error.to_string()))?;
    let dataset = normalize_dataset(&dataset, basepath)?;

    for (index, resource) in dataset.resources.iter().flatten().enumerate() {
        let root_json_pointer = format!("/resources/{index}");
        let root_json_pointer = Some(root_json_pointer.as_str());

        if let Some(PathOr::Path(path)) = &resource.file_dialect {
            let result = validate_file_dialect(path.as_str(), root_json_pointer)?;
            report.errors.extend(result.errors);
        }

        if let Some(PathOr::Path(path)) = &resource.data_schema {
            let result = validate_data_schema(path.as_str(), root_json_pointer)?;
            report.errors.extend(result.errors);
        }

        if let Some(PathOr::Path(path)) = &resource.table_schema {
            let result = validate_table_schema(path.as_str(), root_json_pointer)?;
            report.errors.extend(result.errors);
        }
    }

    let valid = report.errors.is_empty();
    Ok(DatasetValidationResult {
        valid,
        errors: report.errors,
        dataset: valid.then_some(dataset),
    })
}
