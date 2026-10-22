use fairspec_dataset::InferFileDialectOptions;
use fairspec_metadata::{FileDialect, Resource, Result};

use crate::system::get_system;

pub fn infer_file_dialect(
    resource: &Resource,
    options: &InferFileDialectOptions,
) -> Result<Option<FileDialect>> {
    for plugin in &get_system().plugins {
        if let Some(plugin) = plugin.get_dataset_plugin() {
            let result = plugin.infer_file_dialect(resource, options)?;
            if result.is_some() {
                return Ok(result);
            }
        }
    }
    Ok(None)
}
