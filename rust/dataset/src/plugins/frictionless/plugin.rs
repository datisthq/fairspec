use fairspec_metadata::{Dataset, Result, get_file_name, get_is_remote_path};

use super::actions::dataset::save::save_dataset_to_frictionless;
use crate::models::dataset::{SaveDatasetOptions, SaveDatasetResult};
use crate::plugin::{DatasetPlugin, MetadataPlugin};

#[derive(Debug, Clone, Copy, Default)]
pub struct FrictionlessPlugin;

impl MetadataPlugin for FrictionlessPlugin {}

impl DatasetPlugin for FrictionlessPlugin {
    fn save_dataset(
        &self,
        dataset: &Dataset,
        options: &SaveDatasetOptions,
    ) -> Result<Option<SaveDatasetResult>> {
        let target = &options.target;
        if !get_is_datapackage(target) || get_is_remote_path(target) {
            return Ok(None);
        }
        save_dataset_to_frictionless(dataset, target, false).map(Some)
    }
}

pub(crate) fn get_is_datapackage(path: &str) -> bool {
    get_file_name(path).as_deref() == Some("datapackage.json")
}
