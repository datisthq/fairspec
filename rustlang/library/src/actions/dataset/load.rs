use fairspec_metadata::{Descriptor, Result};

use crate::system::get_system;

pub fn load_dataset(source: &str) -> Result<Option<Descriptor>> {
    for plugin in &get_system().plugins {
        if let Some(plugin) = plugin.get_dataset_plugin() {
            let result = plugin.load_dataset(source)?;
            if result.is_some() {
                return Ok(result);
            }
        }
    }
    Ok(None)
}
