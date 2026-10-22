use fairspec_metadata::{DataSchema, RenderDataSchemaOptions, Result};

use crate::system::get_system;

pub fn render_data_schema_as(
    data_schema: &DataSchema,
    options: &RenderDataSchemaOptions,
) -> Result<Option<String>> {
    for plugin in &get_system().plugins {
        let result = plugin
            .get_metadata_plugin()
            .render_data_schema_as(data_schema, options)?;
        if result.is_some() {
            return Ok(result);
        }
    }
    Ok(None)
}
