use fairspec_dataset::{DatasetPlugin, MetadataPlugin};
use fairspec_metadata::{Resource, Result, get_data_records};

use super::actions::table::load::load_inline_table;
use crate::models::table::{LoadTableOptions, Table};
use crate::plugin::TablePlugin;

#[derive(Debug, Clone, Copy, Default)]
pub struct InlinePlugin;

impl MetadataPlugin for InlinePlugin {}

impl DatasetPlugin for InlinePlugin {}

impl TablePlugin for InlinePlugin {
    fn load_table(&self, resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
        if get_data_records(resource).is_none() {
            return Ok(None);
        }
        load_inline_table(resource, options).map(Some)
    }
}
