use fairspec_dataset::{infer_integrity, infer_textual};
use fairspec_metadata::{PathOr, Resource, Result, infer_resource_name, resolve_file_dialect};
use fairspec_table::InferTableSchemaOptions;

use crate::actions::data_schema::infer::infer_data_schema;
use crate::actions::file_dialect::infer::infer_file_dialect;
use crate::actions::table_schema::infer::infer_table_schema;

pub fn infer_resource(
    resource: &Resource,
    resource_number: Option<usize>,
    options: &InferTableSchemaOptions,
) -> Result<Resource> {
    let mut resource = resource.clone();

    if resource.name.as_deref().is_none_or(str::is_empty) {
        resource.name = Some(infer_resource_name(&resource, resource_number));
    }

    if resource.file_dialect.is_none() {
        resource.file_dialect =
            infer_file_dialect(&resource, &Default::default())?.map(PathOr::Value);
    }

    if resource.textual.is_none() && resolve_file_dialect(resource.file_dialect.as_ref())?.is_some()
    {
        resource.textual = Some(infer_textual(&resource, None)?);
    }

    if resource.integrity.is_none() {
        resource.integrity = infer_integrity(&resource, None, options.concurrency)?;
    }

    let is_data_schema_empty = match &resource.data_schema {
        None => true,
        Some(PathOr::Value(data_schema)) => data_schema.is_empty(),
        Some(PathOr::Path(path)) => path.is_empty(),
    };
    if is_data_schema_empty {
        resource.data_schema = infer_data_schema(&resource)?.map(PathOr::Value);
    }

    if resource.table_schema.is_none() {
        resource.table_schema = infer_table_schema(&resource, options)?.map(PathOr::Value);
    }

    Ok(resource)
}
