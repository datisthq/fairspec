use fairspec_metadata::{Resource, Result};
use fairspec_table::{LoadTableOptions, Table};

use crate::system::get_system;

pub fn load_table(resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
    for plugin in &get_system().plugins {
        if let Some(plugin) = plugin.get_table_plugin() {
            let result = plugin.load_table(resource, options)?;
            if result.is_some() {
                return Ok(result);
            }
        }
    }
    Ok(None)
}
