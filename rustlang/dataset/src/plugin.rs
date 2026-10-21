use fairspec_metadata::{
    ConvertDatasetFromOptions, ConvertDatasetToOptions, ConvertTableSchemaFromOptions,
    ConvertTableSchemaToOptions, DataSchema, Dataset, Descriptor, FileDialect,
    RenderDataSchemaOptions, RenderDatasetOptions, RenderTableSchemaOptions, Resource, Result,
    TableSchema,
};

use crate::models::dataset::{SaveDatasetOptions, SaveDatasetResult};
use crate::models::file_dialect::InferFileDialectOptions;

/// Python's `MetadataPlugin`: every hook returns `None` unless a plugin handles it.
pub trait MetadataPlugin: Send + Sync {
    fn render_dataset_as(
        &self,
        _dataset: &Dataset,
        _options: &RenderDatasetOptions,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    fn convert_dataset_to(
        &self,
        _dataset: &Dataset,
        _options: &ConvertDatasetToOptions,
    ) -> Result<Option<Descriptor>> {
        Ok(None)
    }

    fn convert_dataset_from(
        &self,
        _descriptor: &Descriptor,
        _options: &ConvertDatasetFromOptions,
    ) -> Result<Option<Dataset>> {
        Ok(None)
    }

    fn render_data_schema_as(
        &self,
        _data_schema: &DataSchema,
        _options: &RenderDataSchemaOptions,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    fn render_table_schema_as(
        &self,
        _table_schema: &TableSchema,
        _options: &RenderTableSchemaOptions,
    ) -> Result<Option<String>> {
        Ok(None)
    }

    fn convert_table_schema_to(
        &self,
        _table_schema: &TableSchema,
        _options: &ConvertTableSchemaToOptions,
    ) -> Result<Option<Descriptor>> {
        Ok(None)
    }

    fn convert_table_schema_from(
        &self,
        _descriptor: &Descriptor,
        _options: &ConvertTableSchemaFromOptions,
    ) -> Result<Option<TableSchema>> {
        Ok(None)
    }
}

pub trait DatasetPlugin: MetadataPlugin {
    fn load_dataset(&self, _source: &str) -> Result<Option<Descriptor>> {
        Ok(None)
    }

    fn save_dataset(
        &self,
        _dataset: &Dataset,
        _options: &SaveDatasetOptions,
    ) -> Result<Option<SaveDatasetResult>> {
        Ok(None)
    }

    fn infer_file_dialect(
        &self,
        _resource: &Resource,
        _options: &InferFileDialectOptions,
    ) -> Result<Option<FileDialect>> {
        Ok(None)
    }
}
