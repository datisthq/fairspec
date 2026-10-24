use fairspec_metadata::{Result, TableSchema, get_columns};
use indexmap::IndexMap;
use polars::prelude::*;

use super::_shared::helpers::merge_missing_values;
use crate::actions::column::normalize::normalize_column;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::schema::get_polars_schema;
use crate::models::column::ColumnMapping;
use crate::models::schema::SchemaMapping;
use crate::models::table::Table;

pub(crate) const HEAD_ROWS: u32 = 100;

/// Casts every schema column from its source (read as polars typed it) to its schema type;
/// a schema column missing from the source becomes all-null.
pub fn normalize_table(table: Table, table_schema: &TableSchema) -> Result<Table> {
    let head = table.clone().limit(HEAD_ROWS).collect().into_result()?;
    let mapping = SchemaMapping {
        source: get_polars_schema(head.schema()),
        target: table_schema.clone(),
    };
    let exprs: Vec<Expr> = normalize_columns(&mapping)?.into_values().collect();
    Ok(table.select(exprs))
}

pub fn normalize_columns(mapping: &SchemaMapping) -> Result<IndexMap<String, Expr>> {
    let mut exprs = IndexMap::new();
    for column in get_columns(&get_schema_descriptor(&mapping.target))? {
        let polars_column = mapping
            .source
            .columns
            .iter()
            .find(|polars_column| polars_column.name == column.name());
        let expr = match polars_column {
            Some(polars_column) => normalize_column(
                &ColumnMapping {
                    source: polars_column.clone(),
                    target: merge_missing_values(&column, &mapping.target),
                },
                false,
            ),
            None => lit(NULL).alias(column.name()),
        };
        exprs.insert(column.name().to_string(), expr);
    }
    Ok(exprs)
}

pub(crate) fn get_schema_descriptor(table_schema: &TableSchema) -> fairspec_metadata::Descriptor {
    match serde_json::to_value(table_schema) {
        Ok(serde_json::Value::Object(descriptor)) => descriptor,
        _ => fairspec_metadata::Descriptor::new(),
    }
}
