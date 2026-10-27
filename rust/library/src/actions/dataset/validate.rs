use fairspec_metadata::{
    Dataset, Descriptor, Error, FairspecError, FrictionlessKind, Report, Result, assert_dataset,
    convert_descriptor_from_frictionless, create_report, get_is_frictionless_descriptor,
    infer_resource_name, map_frictionless_errors, validate_dataset_descriptor,
};
use serde_json::Value;

use super::foreign_key::validate_dataset_foreign_keys;
use super::load::load_dataset;
use crate::actions::resource::validate::validate_resource;
use crate::helpers::concurrency::run_concurrent_tasks;
use crate::models::dataset::DatasetSource;
use crate::models::table::ValidateTableOptions;
use crate::system::get_system;

pub fn validate_dataset(
    source: impl Into<DatasetSource>,
    options: &ValidateTableOptions,
) -> Result<Report> {
    let loaded = match source.into() {
        DatasetSource::Path(path) => {
            let validate = |dataset: &Dataset| validate_loaded_dataset(dataset.clone(), options);
            for plugin in &get_system().plugins {
                if let Some(plugin) = plugin.get_dataset_plugin()
                    && let Some(report) = plugin.validate_dataset(&path, &validate)?
                {
                    return Ok(report);
                }
            }
            load_dataset_from_plugins(&path)
        }
        DatasetSource::Descriptor(descriptor)
            if get_is_frictionless_descriptor(&descriptor, FrictionlessKind::Dataset) =>
        {
            return validate_frictionless_dataset(&descriptor, options);
        }
        DatasetSource::Descriptor(descriptor) => assert_dataset(&descriptor, None),
        DatasetSource::Dataset(dataset) => Ok(*dataset),
    };
    let dataset = match loaded {
        Ok(dataset) => dataset,
        Err(Error::Invalid { report, .. }) => {
            return Ok(report.unwrap_or_else(|| create_report(Vec::new(), None)));
        }
        Err(error) => return Err(error),
    };

    validate_loaded_dataset(dataset, options)
}

fn validate_frictionless_dataset(
    descriptor: &Descriptor,
    options: &ValidateTableOptions,
) -> Result<Report> {
    let result = validate_dataset_descriptor(descriptor.clone(), None)?;
    let Some(dataset) = result.dataset else {
        return Ok(create_report(result.errors, None));
    };

    let report = validate_loaded_dataset(dataset, options)?;
    let (_, mapping, _) =
        convert_descriptor_from_frictionless(descriptor, FrictionlessKind::Dataset, None);
    Ok(create_report(
        map_frictionless_errors(report.errors, &mapping),
        None,
    ))
}

fn validate_loaded_dataset(mut dataset: Dataset, options: &ValidateTableOptions) -> Result<Report> {
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
