use fairspec_dataset::DatasetPlugin;
use fairspec_metadata::{Resource, Result, TableSchema};

use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::{LoadTableOptions, SaveTableOptions, Table};

/// A dataset plugin that also reads and writes tables; every hook defaults to `Ok(None)`.
pub trait TablePlugin: DatasetPlugin {
    fn load_table(
        &self,
        _resource: &Resource,
        _options: &LoadTableOptions,
    ) -> Result<Option<Table>> {
        Ok(None)
    }

    fn save_table(&self, _table: Table, _options: &SaveTableOptions) -> Result<Option<String>> {
        Ok(None)
    }

    fn infer_table_schema(
        &self,
        _resource: &Resource,
        _options: &InferTableSchemaOptions,
    ) -> Result<Option<TableSchema>> {
        Ok(None)
    }
}
