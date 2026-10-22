use fairspec_metadata::{Result, TableSchema, get_columns};
use indexmap::IndexMap;
use polars::prelude::*;

use super::_shared::helpers::merge_missing_values;
use super::normalize::{HEAD_ROWS, get_schema_descriptor};
use crate::actions::column::denormalize::denormalize_column;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::schema::get_polars_schema;
use crate::models::column::{ColumnMapping, DenormalizeColumnOptions};
use crate::models::schema::SchemaMapping;
use crate::models::table::Table;

pub fn denormalize_table(
    table: Table,
    table_schema: &TableSchema,
    options: &DenormalizeColumnOptions,
) -> Result<Table> {
    let head = table.clone().limit(HEAD_ROWS).collect().into_result()?;
    let mapping = SchemaMapping {
        source: get_polars_schema(head.schema()),
        target: table_schema.clone(),
    };
    let exprs: Vec<Expr> = denormalize_columns(&mapping, options)?
        .into_values()
        .collect();
    Ok(table.select(exprs))
}

pub fn denormalize_columns(
    mapping: &SchemaMapping,
    options: &DenormalizeColumnOptions,
) -> Result<IndexMap<String, Expr>> {
    let mut exprs = IndexMap::new();
    for column in get_columns(&get_schema_descriptor(&mapping.target))? {
        let polars_column = mapping
            .source
            .columns
            .iter()
            .find(|polars_column| polars_column.name == column.name());
        let expr = match polars_column {
            Some(polars_column) => denormalize_column(
                &ColumnMapping {
                    source: polars_column.clone(),
                    target: merge_missing_values(&column, &mapping.target),
                },
                options,
            ),
            None => lit(NULL).alias(column.name()),
        };
        exprs.insert(column.name().to_string(), expr);
    }
    Ok(exprs)
}
