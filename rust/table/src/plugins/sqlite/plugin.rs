use fairspec_dataset::{DatasetPlugin, InferFileDialectOptions, MetadataPlugin};
use fairspec_metadata::{FileDialect, PathOr, Resource, Result, get_supported_file_dialect};
use serde_json::{Value, json};

use super::actions::table::load::load_sqlite_table;
use super::actions::table::save::save_sqlite_table;
use crate::models::table::{LoadTableOptions, SaveTableOptions, Table};
use crate::plugin::TablePlugin;

#[derive(Debug, Clone, Copy, Default)]
pub struct SqlitePlugin;

impl MetadataPlugin for SqlitePlugin {}

impl DatasetPlugin for SqlitePlugin {
    fn infer_file_dialect(
        &self,
        resource: &Resource,
        _options: &InferFileDialectOptions,
    ) -> Result<Option<FileDialect>> {
        if get_supported_file_dialect(resource, &["sqlite"])?.is_none() {
            return Ok(None);
        }
        Ok(serde_json::from_value(json!({"format": "sqlite"})).ok())
    }
}

impl TablePlugin for SqlitePlugin {
    fn load_table(&self, resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
        if get_supported_file_dialect(resource, &["sqlite"])?.is_none() {
            return Ok(None);
        }
        load_sqlite_table(resource, options).map(Some)
    }

    fn save_table(&self, table: Table, options: &SaveTableOptions) -> Result<Option<String>> {
        let resource = Resource {
            data: Some(Value::String(options.path.clone())),
            file_dialect: options.file_dialect.clone().map(PathOr::Value),
            ..Resource::default()
        };
        if get_supported_file_dialect(&resource, &["sqlite"])?.is_none() {
            return Ok(None);
        }
        save_sqlite_table(table, options).map(Some)
    }
}
