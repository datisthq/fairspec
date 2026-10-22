use fairspec_dataset::prefetch_files;
use fairspec_metadata::{Error, Resource, Result, resolve_table_schema};
use polars::prelude::*;

use crate::actions::table::normalize::normalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::models::table::{LoadTableOptions, Table};

pub fn load_parquet_table(resource: &Resource, options: &LoadTableOptions) -> Result<Table> {
    let paths = prefetch_files(
        resource,
        None,
        options.infer_table_schema_options.concurrency,
    )?;
    if paths.is_empty() {
        return Err(Error::Load("Resource data is not defined".to_string()));
    }

    let tables = paths
        .iter()
        .map(|path| {
            LazyFrame::scan_parquet(PlRefPath::new(path), ScanArgsParquet::default()).into_result()
        })
        .collect::<Result<Vec<_>>>()?;
    let mut table = match tables.len() {
        1 => tables.into_iter().next().unwrap_or_default(),
        _ => concat(tables, UnionArgs::default()).into_result()?,
    };

    if options.denormalized != Some(true) {
        let table_schema = match resolve_table_schema(resource.table_schema.as_ref())? {
            Some(table_schema) => table_schema,
            None => {
                infer_table_schema_from_table(table.clone(), &options.infer_table_schema_options)?
            }
        };
        table = normalize_table(table, &table_schema)?;
    }

    Ok(table)
}
