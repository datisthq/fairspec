use fairspec_metadata::{Dataset, Descriptor, Result, get_file_extension};

use super::actions::dataset::load::load_dataset_from_zip;
use super::actions::dataset::save::save_dataset_to_zip;
use crate::models::dataset::{SaveDatasetOptions, SaveDatasetResult};
use crate::plugin::{DatasetPlugin, MetadataPlugin};
use crate::plugins::descriptor::plugin::to_descriptor;

#[derive(Debug, Clone, Copy, Default)]
pub struct ZipPlugin;

impl MetadataPlugin for ZipPlugin {}

impl DatasetPlugin for ZipPlugin {
    fn load_dataset(&self, source: &str) -> Result<Option<Descriptor>> {
        if !get_is_zip(source) {
            return Ok(None);
        }
        let dataset = load_dataset_from_zip(source)?;
        to_descriptor(&dataset).map(Some)
    }

    fn save_dataset(
        &self,
        dataset: &Dataset,
        options: &SaveDatasetOptions,
    ) -> Result<Option<SaveDatasetResult>> {
        if !get_is_zip(&options.target) {
            return Ok(None);
        }
        save_dataset_to_zip(dataset, &options.target, options.with_remote)?;
        Ok(Some(SaveDatasetResult { path: None }))
    }
}

fn get_is_zip(path: &str) -> bool {
    get_file_extension(path).as_deref() == Some("zip")
}
