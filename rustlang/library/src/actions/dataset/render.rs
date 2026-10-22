use fairspec_metadata::{Dataset, RenderDatasetOptions, Result};

use crate::system::get_system;

pub fn render_dataset_as(
    dataset: &Dataset,
    options: &RenderDatasetOptions,
) -> Result<Option<String>> {
    for plugin in &get_system().plugins {
        let result = plugin
            .get_metadata_plugin()
            .render_dataset_as(dataset, options)?;
        if result.is_some() {
            return Ok(result);
        }
    }
    Ok(None)
}
