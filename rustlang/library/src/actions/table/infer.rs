use fairspec_metadata::{PathOr, Resource, Result, resolve_table_schema};
use fairspec_table::{DenormalizeColumnOptions, LoadTableOptions, Table, denormalize_table};

use super::load::load_table;
use crate::actions::file_dialect::infer::infer_file_dialect;
use crate::actions::table_schema::infer::infer_table_schema;

pub fn infer_table(resource: &Resource, options: &LoadTableOptions) -> Result<Option<Table>> {
    let mut resource = resource.clone();
    if resource.file_dialect.is_none() {
        resource.file_dialect =
            infer_file_dialect(&resource, &Default::default())?.map(PathOr::Value);
    }

    let load_options = LoadTableOptions {
        denormalized: Some(true),
        ..options.clone()
    };
    let Some(mut table) = load_table(&resource, &load_options)? else {
        return Ok(None);
    };

    let mut table_schema = resolve_table_schema(resource.table_schema.as_ref())?;
    if table_schema.is_none() {
        table_schema = infer_table_schema(&resource, &options.infer_table_schema_options)?;
    }

    if let Some(table_schema) = table_schema {
        table = denormalize_table(table, &table_schema, &DenormalizeColumnOptions::default())?;
    }
    Ok(Some(table))
}
