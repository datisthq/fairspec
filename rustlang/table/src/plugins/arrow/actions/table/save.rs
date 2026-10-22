use std::fs::File;

use fairspec_dataset::assert_local_path_vacant;
use fairspec_metadata::Result;
use polars::prelude::*;

use crate::actions::table::denormalize::denormalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::models::column::DenormalizeColumnOptions;
use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::{SaveTableOptions, Table};
use crate::plugins::arrow::settings::NATIVE_TYPES;

/// Collects the table before writing it, where Python's twin streams it through a sink.
pub fn save_arrow_table(table: Table, options: &SaveTableOptions) -> Result<String> {
    let path = &options.path;
    if options.overwrite != Some(true) {
        assert_local_path_vacant(path)?;
    }

    let table_schema = match &options.table_schema {
        Some(table_schema) => table_schema.clone(),
        None => infer_table_schema_from_table(
            table.clone(),
            &InferTableSchemaOptions {
                table_schema_options: options.table_schema_options.clone(),
                keep_strings: Some(true),
                ..InferTableSchemaOptions::default()
            },
        )?,
    };
    let denormalize_options = DenormalizeColumnOptions {
        native_types: Some(NATIVE_TYPES.iter().map(|value| value.to_string()).collect()),
    };
    let table = denormalize_table(table, &table_schema, &denormalize_options)?;

    let mut frame = table.collect().into_result()?;
    let file = File::create(path)?;
    IpcWriter::new(file).finish(&mut frame).into_result()?;

    Ok(path.clone())
}
