use fairspec_dataset::{DatasetPlugin, InferFileDialectOptions, MetadataPlugin};
use fairspec_metadata::{FileDialect, PathOr, Resource, Result, get_supported_file_dialect};
use serde_json::Value;

use super::actions::file_dialect::infer::infer_json_file_dialect;
use super::actions::table::load::load_json_table;
use super::actions::table::save::save_json_table;
use crate::models::table::{LoadTableOptions, SaveTableOptions, Table};
use crate::plugin::TablePlugin;

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonPlugin;

impl MetadataPlugin for JsonPlugin {}

impl DatasetPlugin for JsonPlugin {
    fn infer_file_dialect(
        &self,
        resource: &Resource,
        _options: &InferFileDialectOptions,
    ) -> Result<Option<FileDialect>> {
        if get_supported_file_dialect(resource, &["json", "jsonl"])?.is_none() {
            return Ok(None);
        }
        infer_json_file_dialect(resource, None)
    }
}

impl TablePlugin for JsonPlugin {
    fn load_table(&self, resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
        if get_supported_file_dialect(resource, &["json", "jsonl"])?.is_none() {
            return Ok(None);
        }
        load_json_table(resource, options)
    }

    fn save_table(&self, table: Table, options: &SaveTableOptions) -> Result<Option<String>> {
        let resource = Resource {
            data: Some(Value::String(options.path.clone())),
            file_dialect: options.file_dialect.clone().map(PathOr::Value),
            ..Resource::default()
        };
        if get_supported_file_dialect(&resource, &["json", "jsonl"])?.is_none() {
            return Ok(None);
        }
        save_json_table(table, options).map(Some)
    }
}
