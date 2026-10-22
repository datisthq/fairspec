use fairspec_dataset::{SaveDatasetOptions, SaveDatasetResult};
use fairspec_metadata::{Dataset, Result};

use crate::system::get_system;

pub fn save_dataset(
    dataset: &Dataset,
    options: &SaveDatasetOptions,
) -> Result<Option<SaveDatasetResult>> {
    for plugin in &get_system().plugins {
        if let Some(plugin) = plugin.get_dataset_plugin() {
            let result = plugin.save_dataset(dataset, options)?;
            if result.is_some() {
                return Ok(result);
            }
        }
    }
    Ok(None)
}
