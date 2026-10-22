use fairspec_metadata::{Resource, Result, TableSchema};
use fairspec_table::{InferTableSchemaOptions, LoadTableOptions, infer_table_schema_from_table};

use crate::actions::table::load::load_table;
use crate::system::get_system;

pub fn infer_table_schema(
    resource: &Resource,
    options: &InferTableSchemaOptions,
) -> Result<Option<TableSchema>> {
    for plugin in &get_system().plugins {
        if let Some(plugin) = plugin.get_table_plugin() {
            let result = plugin.infer_table_schema(resource, options)?;
            if result.is_some() {
                return Ok(result);
            }
        }
    }

    let load_options = LoadTableOptions {
        denormalized: Some(true),
        ..Default::default()
    };
    match load_table(resource, &load_options)? {
        Some(table) => infer_table_schema_from_table(table, options).map(Some),
        None => Ok(None),
    }
}
