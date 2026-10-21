use std::path::Path;

use fairspec_metadata::{Dataset, Descriptor, Result, get_is_remote_path};

use super::actions::dataset::load::load_dataset_from_folder;
use super::actions::dataset::save::save_dataset_to_folder;
use crate::models::dataset::{SaveDatasetOptions, SaveDatasetResult};
use crate::plugin::{DatasetPlugin, MetadataPlugin};
use crate::plugins::descriptor::plugin::to_descriptor;

#[derive(Debug, Clone, Copy, Default)]
pub struct FolderPlugin;

impl MetadataPlugin for FolderPlugin {}

impl DatasetPlugin for FolderPlugin {
    fn load_dataset(&self, source: &str) -> Result<Option<Descriptor>> {
        if !get_is_folder(source) {
            return Ok(None);
        }
        let dataset = load_dataset_from_folder(source)?;
        to_descriptor(&dataset).map(Some)
    }

    fn save_dataset(
        &self,
        dataset: &Dataset,
        options: &SaveDatasetOptions,
    ) -> Result<Option<SaveDatasetResult>> {
        let target = &options.target;
        if !get_is_folder_target(target) {
            return Ok(None);
        }
        save_dataset_to_folder(dataset, target, options.with_remote)?;
        Ok(Some(SaveDatasetResult {
            path: Some(target.clone()),
        }))
    }
}

fn get_is_folder(path: &str) -> bool {
    !get_is_remote_path(path) && Path::new(path).is_dir()
}

fn get_is_folder_target(path: &str) -> bool {
    if get_is_remote_path(path) {
        return false;
    }
    get_is_folder(path) || Path::new(path).extension().is_none()
}
