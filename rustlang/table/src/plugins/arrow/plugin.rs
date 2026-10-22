use fairspec_dataset::{DatasetPlugin, MetadataPlugin};
use fairspec_metadata::{PathOr, Resource, Result, get_supported_file_dialect};
use serde_json::Value;

use super::actions::table::load::load_arrow_table;
use super::actions::table::save::save_arrow_table;
use crate::models::table::{LoadTableOptions, SaveTableOptions, Table};
use crate::plugin::TablePlugin;

#[derive(Debug, Clone, Copy, Default)]
pub struct ArrowPlugin;

impl MetadataPlugin for ArrowPlugin {}

impl DatasetPlugin for ArrowPlugin {}

impl TablePlugin for ArrowPlugin {
    fn load_table(&self, resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
        if get_supported_file_dialect(resource, &["arrow"])?.is_none() {
            return Ok(None);
        }
        load_arrow_table(resource, options).map(Some)
    }

    fn save_table(&self, table: Table, options: &SaveTableOptions) -> Result<Option<String>> {
        let resource = Resource {
            data: Some(Value::String(options.path.clone())),
            file_dialect: options.file_dialect.clone().map(PathOr::Value),
            ..Resource::default()
        };
        if get_supported_file_dialect(&resource, &["arrow"])?.is_none() {
            return Ok(None);
        }
        save_arrow_table(table, options).map(Some)
    }
}
