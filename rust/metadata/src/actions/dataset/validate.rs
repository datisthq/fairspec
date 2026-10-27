use std::collections::HashSet;

use serde_json::Value;

use crate::actions::data_schema::validate::validate_data_schema;
use crate::actions::descriptor::inspect::inspect_model;
use crate::actions::descriptor::load::load_descriptor_source;
use crate::actions::descriptor::validate::validate_profiled_descriptor;
use crate::actions::file_dialect::validate::validate_file_dialect;
use crate::actions::frictionless::convert::{FrictionlessKind, prepare_frictionless_descriptor};
use crate::actions::frictionless::mapping::{FrictionlessMapping, map_frictionless_errors};
use crate::actions::resource::data::get_data_paths;
use crate::actions::table_schema::inspect::{inspect_table_schema, inspect_table_schema_nulls};
use crate::actions::table_schema::validate::validate_table_schema;
use crate::models::dataset::{Dataset, DatasetValidationResult};
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::profile::ProfileType;

use super::normalize::normalize_dataset;

pub fn validate_dataset_descriptor(
    source: impl Into<PathOr<Descriptor>>,
    basepath: Option<&str>,
) -> Result<DatasetValidationResult> {
    let loaded = load_descriptor_source(source)?;
    let (prepared, mapping, errors) =
        prepare_frictionless_descriptor(loaded, FrictionlessKind::Dataset, None);
    let Some(prepared) = prepared else {
        return Ok(DatasetValidationResult {
            valid: false,
            errors,
            dataset: None,
        });
    };

    let (descriptor, mut report) =
        validate_profiled_descriptor(prepared, ProfileType::Dataset, None)?;

    if !report.valid {
        return Ok(DatasetValidationResult {
            valid: report.valid,
            errors: map_errors(report.errors, mapping.as_ref()),
            dataset: None,
        });
    }

    let (dataset, errors) = inspect_model::<Dataset>(&descriptor, None);
    let Some(dataset) = dataset else {
        return Ok(DatasetValidationResult {
            valid: false,
            errors: map_errors([report.errors, errors].concat(), mapping.as_ref()),
            dataset: None,
        });
    };
    let dataset = normalize_dataset(&dataset, basepath)?;

    let mut names = HashSet::new();
    for (index, resource) in dataset.resources.iter().flatten().enumerate() {
        let root_json_pointer = format!("/resources/{index}");
        if let Some(name) = &resource.name
            && !names.insert(name.as_str())
        {
            report.errors.push(FairspecError::Metadata(MetadataError {
                message: format!("resource name \"{name}\" is not unique"),
                json_pointer: format!("{root_json_pointer}/name"),
                ..Default::default()
            }));
        }
        if resource.data_schema.is_some() && get_data_paths(resource).len() > 1 {
            report.errors.push(FairspecError::Metadata(MetadataError {
                message: "a data schema needs single-part data".to_string(),
                json_pointer: format!("{root_json_pointer}/dataSchema"),
                ..Default::default()
            }));
        }
        let root_json_pointer = Some(root_json_pointer.as_str());

        if let Some(PathOr::Path(path)) = &resource.file_dialect {
            let result = validate_file_dialect(path.as_str(), root_json_pointer)?;
            report.errors.extend(result.errors);
        }

        if let Some(PathOr::Path(path)) = &resource.data_schema {
            let result = validate_data_schema(path.as_str(), root_json_pointer)?;
            report.errors.extend(result.errors);
        }

        match &resource.table_schema {
            Some(PathOr::Path(path)) => {
                let result = validate_table_schema(path.as_str(), root_json_pointer)?;
                report.errors.extend(result.errors);
            }
            Some(PathOr::Value(table_schema)) => {
                let pointer = format!("/resources/{index}/tableSchema");
                report
                    .errors
                    .extend(inspect_table_schema(table_schema, Some(&pointer)));
                report.errors.extend(inspect_table_schema_nulls(
                    &get_table_schema_descriptor(&descriptor, index),
                    Some(&pointer),
                ));
            }
            None => {}
        }
    }

    let valid = report.errors.is_empty();
    Ok(DatasetValidationResult {
        valid,
        errors: map_errors(report.errors, mapping.as_ref()),
        dataset: valid.then_some(dataset),
    })
}

fn map_errors(
    errors: Vec<FairspecError>,
    mapping: Option<&FrictionlessMapping>,
) -> Vec<FairspecError> {
    match mapping {
        Some(mapping) => map_frictionless_errors(errors, mapping),
        None => errors,
    }
}

fn get_table_schema_descriptor(descriptor: &Descriptor, index: usize) -> Descriptor {
    descriptor
        .get("resources")
        .and_then(|resources| resources.get(index))
        .and_then(|resource| resource.get("tableSchema"))
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default()
}
