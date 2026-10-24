use fairspec_metadata::{RenderTableSchemaOptions, Result, TableSchema};

use crate::system::get_system;

pub fn render_table_schema_as(
    table_schema: &TableSchema,
    options: &RenderTableSchemaOptions,
) -> Result<Option<String>> {
    for plugin in &get_system().plugins {
        let result = plugin
            .get_metadata_plugin()
            .render_table_schema_as(table_schema, options)?;
        if result.is_some() {
            return Ok(result);
        }
    }
    Ok(None)
}
