use std::io::Cursor;

use fairspec_metadata::{Error, Resource, Result, get_data_records, resolve_table_schema};
use polars::prelude::*;
use serde_json::Value;

use crate::actions::table::normalize::normalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::models::table::{LoadTableOptions, Table};

pub fn load_inline_table(resource: &Resource, options: &LoadTableOptions) -> Result<Table> {
    let Some(records) = get_data_records(resource) else {
        return Err(Error::Load(
            "Resource data is not defined or tabular".to_string(),
        ));
    };

    let text = serde_json::to_vec(&Value::Array(records.to_vec()))
        .map_err(|error| Error::Parse(error.to_string()))?;
    let mut table = JsonReader::new(Cursor::new(text))
        .infer_schema_len(None)
        .finish()
        .into_result()?
        .lazy();

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
