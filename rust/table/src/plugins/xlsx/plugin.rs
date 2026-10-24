use fairspec_dataset::{DatasetPlugin, InferFileDialectOptions, MetadataPlugin};
use fairspec_metadata::{FileDialect, PathOr, Resource, Result, get_supported_file_dialect};
use serde_json::Value;

use super::actions::file_dialect::infer::infer_xlsx_file_dialect;
use super::actions::table::load::load_xlsx_table;
use super::actions::table::save::save_xlsx_table;
use crate::models::table::{LoadTableOptions, SaveTableOptions, Table};
use crate::plugin::TablePlugin;

#[derive(Debug, Clone, Copy, Default)]
pub struct XlsxPlugin;

impl MetadataPlugin for XlsxPlugin {}

impl DatasetPlugin for XlsxPlugin {
    fn infer_file_dialect(
        &self,
        resource: &Resource,
        _options: &InferFileDialectOptions,
    ) -> Result<Option<FileDialect>> {
        if get_supported_file_dialect(resource, &["xlsx", "ods"])?.is_none() {
            return Ok(None);
        }
        infer_xlsx_file_dialect(resource, None)
    }
}

impl TablePlugin for XlsxPlugin {
    fn load_table(&self, resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
        let Some(file_dialect) = get_supported_file_dialect(resource, &["xlsx", "ods"])? else {
            return Ok(None);
        };
        let resource = Resource {
            file_dialect: Some(PathOr::Value(file_dialect)),
            ..resource.clone()
        };
        load_xlsx_table(&resource, options).map(Some)
    }

    fn save_table(&self, table: Table, options: &SaveTableOptions) -> Result<Option<String>> {
        let resource = Resource {
            data: Some(Value::String(options.path.clone())),
            file_dialect: options.file_dialect.clone().map(PathOr::Value),
            ..Resource::default()
        };
        if get_supported_file_dialect(&resource, &["xlsx", "ods"])?.is_none() {
            return Ok(None);
        }
        save_xlsx_table(table, options).map(Some)
    }
}
