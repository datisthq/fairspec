use fairspec_metadata::Result;
use fairspec_table::{SaveTableOptions, Table};

use crate::system::get_system;

pub fn save_table(table: Table, options: &SaveTableOptions) -> Result<Option<String>> {
    for plugin in &get_system().plugins {
        if let Some(plugin) = plugin.get_table_plugin() {
            let result = plugin.save_table(table.clone(), options)?;
            if result.is_some() {
                return Ok(result);
            }
        }
    }
    Ok(None)
}
