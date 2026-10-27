use fairspec_metadata::{
    Dataset, Descriptor, Error, FrictionlessKind, Report, Result,
    convert_descriptor_from_frictionless, create_report, get_file_extension,
    get_is_frictionless_descriptor, get_is_remote_path, load_dataset_descriptor, load_descriptor,
    map_frictionless_errors, resolve_basepath, save_dataset_descriptor,
    validate_dataset_descriptor,
};
use serde_json::Value;

use crate::models::dataset::{SaveDatasetOptions, SaveDatasetResult};
use crate::plugin::{DatasetPlugin, MetadataPlugin};

#[derive(Debug, Clone, Copy, Default)]
pub struct DescriptorPlugin;

impl MetadataPlugin for DescriptorPlugin {}

impl DatasetPlugin for DescriptorPlugin {
    fn load_dataset(&self, source: &str) -> Result<Option<Descriptor>> {
        if !get_is_json(source) {
            return Ok(None);
        }
        let dataset = load_dataset_descriptor(source)?;
        to_descriptor(&dataset).map(Some)
    }

    fn validate_dataset(
        &self,
        source: &str,
        validate_dataset: &dyn Fn(&Dataset) -> Result<Report>,
    ) -> Result<Option<Report>> {
        if !get_is_json(source) {
            return Ok(None);
        }

        let basepath = resolve_basepath(source)?;
        let descriptor = load_descriptor(source, false)?;

        let result = validate_dataset_descriptor(descriptor.clone(), Some(&basepath))?;
        let Some(dataset) = result.dataset else {
            return Ok(Some(create_report(result.errors, None)));
        };

        let report = validate_dataset(&dataset)?;
        if !get_is_frictionless_descriptor(&descriptor, FrictionlessKind::Dataset) {
            return Ok(Some(report));
        }

        let (_, mapping, _) =
            convert_descriptor_from_frictionless(&descriptor, FrictionlessKind::Dataset, None);
        let errors = map_frictionless_errors(report.errors, &mapping);
        Ok(Some(create_report(errors, None)))
    }

    fn save_dataset(
        &self,
        dataset: &Dataset,
        options: &SaveDatasetOptions,
    ) -> Result<Option<SaveDatasetResult>> {
        let target = &options.target;
        if !get_is_local_json(target) {
            return Ok(None);
        }
        save_dataset_descriptor(dataset, target, false)?;
        Ok(Some(SaveDatasetResult {
            path: Some(target.clone()),
        }))
    }
}

pub(crate) fn to_descriptor(dataset: &Dataset) -> Result<Descriptor> {
    match serde_json::to_value(dataset) {
        Ok(Value::Object(descriptor)) => Ok(descriptor),
        Ok(_) => Ok(Descriptor::new()),
        Err(error) => Err(Error::Parse(error.to_string())),
    }
}

fn get_is_json(path: &str) -> bool {
    get_file_extension(path).as_deref() == Some("json")
}

fn get_is_local_json(path: &str) -> bool {
    get_is_json(path) && !get_is_remote_path(path)
}
