use fairspec_metadata::{
    Dataset, Descriptor, Error, Result, get_file_extension, get_is_remote_path,
    load_dataset_descriptor, save_dataset_descriptor,
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

    fn save_dataset(
        &self,
        dataset: &Dataset,
        options: &SaveDatasetOptions,
    ) -> Result<Option<SaveDatasetResult>> {
        let target = &options.target;
        if !get_is_local_json(target) || !target.ends_with("datapackage.json") {
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
