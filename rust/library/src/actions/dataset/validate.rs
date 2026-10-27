use fairspec_metadata::{
    Dataset, Error, FairspecError, Report, Result, assert_dataset, create_report,
    infer_resource_name,
};
use serde_json::Value;

use super::foreign_key::validate_dataset_foreign_keys;
use super::load::load_dataset;
use crate::actions::resource::validate::validate_resource;
use crate::helpers::concurrency::run_concurrent_tasks;
use crate::models::dataset::DatasetSource;
use crate::models::table::ValidateTableOptions;

pub fn validate_dataset(
    source: impl Into<DatasetSource>,
    options: &ValidateTableOptions,
) -> Result<Report> {
    let loaded = match source.into() {
        DatasetSource::Path(path) => load_dataset_from_plugins(&path),
        DatasetSource::Descriptor(descriptor) => assert_dataset(&descriptor, None),
        DatasetSource::Dataset(dataset) => Ok(*dataset),
    };
    let mut dataset = match loaded {
        Ok(dataset) => dataset,
        Err(Error::Invalid { report, .. }) => {
            return Ok(report.unwrap_or_else(|| create_report(Vec::new(), None)));
        }
        Err(error) => return Err(error),
    };

    let mut errors = validate_dataset_resources(&mut dataset, options)?;
    let fk_report = validate_dataset_foreign_keys(&dataset, options)?;
    errors.extend(fk_report.errors);
    Ok(create_report(errors, None))
}

fn load_dataset_from_plugins(path: &str) -> Result<Dataset> {
    let Some(descriptor) = load_dataset(path)? else {
        return Err(Error::Load(format!(
            "No plugin can load the dataset: {path}"
        )));
    };
    serde_json::from_value(Value::Object(descriptor))
        .map_err(|error| Error::Parse(error.to_string()))
}

// The inferred names stay on the dataset, so the foreign key errors carry them too
fn validate_dataset_resources(
    dataset: &mut Dataset,
    options: &ValidateTableOptions,
) -> Result<Vec<FairspecError>> {
    let resources = dataset.resources.get_or_insert_default();
    for (index, resource) in resources.iter_mut().enumerate() {
        if resource.name.as_deref().is_none_or(str::is_empty) {
            resource.name = Some(infer_resource_name(resource, Some(index + 1)));
        }
    }
    let resources = resources.clone();

    let validate = |resource: fairspec_metadata::Resource| -> Result<Vec<FairspecError>> {
        let report = validate_resource(&resource, options)?;
        let name = resource.name.clone().unwrap_or_default();
        report
            .errors
            .into_iter()
            .map(|error| set_error_resource_name(error, &name))
            .collect()
    };

    let concurrency = options
        .load_table_options
        .infer_table_schema_options
        .concurrency;
    let mut errors = Vec::new();
    for resource_errors in run_concurrent_tasks(validate, resources, concurrency) {
        errors.extend(resource_errors?);
    }
    Ok(errors)
}

// Every error variant has a `resourceName`; going through JSON spares a match per variant
fn set_error_resource_name(error: FairspecError, name: &str) -> Result<FairspecError> {
    let mut value = serde_json::to_value(error).map_err(|error| Error::Parse(error.to_string()))?;
    if let Value::Object(object) = &mut value {
        object.insert("resourceName".to_string(), Value::String(name.to_string()));
    }
    serde_json::from_value(value).map_err(|error| Error::Parse(error.to_string()))
}
