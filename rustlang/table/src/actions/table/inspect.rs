use std::collections::{HashMap, HashSet};

use fairspec_metadata::{
    ColumnError, ColumnExtraError, ColumnMissingError, Result, TableError, TableSchema, get_columns,
};
use polars::prelude::*;

use super::_shared::helpers::merge_missing_values;
use super::checks::key::{
    KeyHashProbe, RowKeyCheck, create_key_hash_probe, create_row_key_checks, create_row_key_error,
};
use super::normalize::get_schema_descriptor;
use crate::actions::check::inspect::inspect_table_checks;
use crate::actions::column::inspect::inspect_columns;
use crate::helpers::error::PolarsResultExt;
use crate::helpers::schema::get_polars_schema;
use crate::models::check::TableCheck;
use crate::models::column::ColumnMapping;
use crate::models::schema::SchemaMapping;
use crate::models::table::Table;
use crate::settings::{ERROR_COLUMN_NAME, NUMBER_COLUMN_NAME};

/// Validates a table against its schema (columns, cells, keys) and the table checks.
/// `sample_rows` defaults to 100 and `max_errors` to 1000.
pub fn inspect_table(
    table: Table,
    table_schema: Option<&TableSchema>,
    checks: Option<&[TableCheck]>,
    sample_rows: Option<usize>,
    max_errors: Option<usize>,
    concurrency: Option<usize>,
) -> Result<Vec<TableError>> {
    let sample_rows = sample_rows.unwrap_or(100);
    let max_errors = max_errors.unwrap_or(1000);
    let mut errors = Vec::new();
    let mut mapping = None;

    if let Some(table_schema) = table_schema {
        let sample = table
            .clone()
            .limit(sample_rows as IdxSize)
            .collect()
            .into_result()?;
        let schema_mapping = SchemaMapping {
            source: get_polars_schema(sample.schema()),
            target: table_schema.clone(),
        };

        let key_checks = create_row_key_checks(&schema_mapping)?;
        let key_probes: Vec<KeyHashProbe> = key_checks.iter().map(create_key_hash_probe).collect();
        let flag_exprs: Vec<Expr> = key_probes.iter().map(|p| p.flag_expr.clone()).collect();
        errors.extend(inspect_columns_of_table(
            &schema_mapping,
            table.clone(),
            max_errors,
            &flag_exprs,
        )?);

        // The cell checks stop reading at max_errors, leaving the key hashes partial, but
        // only once their errors fill the report, which then has no room for row errors
        if errors.len() < max_errors {
            errors.extend(inspect_rows(
                &schema_mapping,
                table.clone(),
                key_checks,
                key_probes,
                max_errors,
                concurrency,
            )?);
        }
        mapping = Some(schema_mapping);
    }

    if let Some(checks) = checks.filter(|checks| !checks.is_empty())
        && errors.len() < max_errors
    {
        errors.extend(inspect_table_checks(
            table,
            checks,
            mapping.as_ref(),
            max_errors,
        )?);
    }

    errors.truncate(max_errors);
    Ok(errors)
}

fn inspect_columns_of_table(
    mapping: &SchemaMapping,
    table: Table,
    max_errors: usize,
    flag_exprs: &[Expr],
) -> Result<Vec<TableError>> {
    let mut errors = Vec::new();
    let target = &mapping.target;
    let columns = get_columns(&get_schema_descriptor(target))?;

    let mut missing_errors: HashMap<String, Vec<TableError>> = HashMap::new();
    let mut column_mappings = Vec::new();
    for column in &columns {
        let polars_column = mapping
            .source
            .columns
            .iter()
            .find(|polars_column| polars_column.name == column.name());

        let Some(polars_column) = polars_column else {
            let is_required = target.all_required == Some(true)
                || target
                    .required
                    .as_ref()
                    .is_some_and(|required| required.iter().any(|name| name == column.name()));
            let column_errors = if is_required {
                vec![TableError::Column(ColumnError::Missing(
                    ColumnMissingError {
                        resource_name: None,
                        column_name: column.name().to_string(),
                    },
                ))]
            } else {
                Vec::new()
            };
            missing_errors.insert(column.name().to_string(), column_errors);
            continue;
        };

        column_mappings.push(ColumnMapping {
            source: polars_column.clone(),
            target: merge_missing_values(column, target),
        });
    }

    let mut cell_errors = inspect_columns(&column_mappings, table, max_errors, flag_exprs)?;
    for column in &columns {
        let column_errors = match missing_errors.remove(column.name()) {
            Some(column_errors) if !column_errors.is_empty() => column_errors,
            _ => cell_errors.shift_remove(column.name()).unwrap_or_default(),
        };
        errors.extend(column_errors);
    }

    if target.additional_properties == Some(false) {
        let names: HashSet<&str> = columns.iter().map(|column| column.name()).collect();
        for polars_column in &mapping.source.columns {
            if !names.contains(polars_column.name.as_str()) {
                errors.push(TableError::Column(ColumnError::Extra(ColumnExtraError {
                    resource_name: None,
                    column_name: polars_column.name.clone(),
                })));
            }
        }
    }

    errors.sort_by_key(|error| !matches!(error, TableError::Column(_)));
    Ok(errors)
}

fn inspect_rows(
    mapping: &SchemaMapping,
    table: Table,
    key_checks: Vec<RowKeyCheck>,
    key_probes: Vec<KeyHashProbe>,
    max_errors: usize,
    concurrency: Option<usize>,
) -> Result<Vec<TableError>> {
    let mut errors = Vec::new();
    let columns = get_columns(&get_schema_descriptor(&mapping.target))?;
    let max_row_errors = if columns.is_empty() {
        max_errors
    } else {
        max_errors.div_ceil(columns.len())
    };

    let inspect = |(check, probe): (RowKeyCheck, KeyHashProbe)| -> Result<Vec<TableError>> {
        if !probe.get_has_duplicate_key() {
            return Ok(Vec::new());
        }

        // A literal is broadcast to every row, so flag the error with a boolean rather
        // than the error itself, which would materialize a full-length column
        let frame = table
            .clone()
            .with_row_index(NUMBER_COLUMN_NAME, Some(1))
            .with_column(
                when(check.is_error_expr.clone())
                    .then(lit(true))
                    .otherwise(lit(NULL).cast(DataType::Boolean))
                    .alias(ERROR_COLUMN_NAME),
            )
            .filter(col(ERROR_COLUMN_NAME).is_not_null())
            .select([col(NUMBER_COLUMN_NAME).cast(DataType::UInt64)])
            .limit(max_row_errors as IdxSize)
            .collect()
            .into_result()?;

        let numbers = frame.column(NUMBER_COLUMN_NAME).into_result()?;
        let numbers = numbers.u64().into_result()?;
        Ok(numbers
            .into_no_null_iter()
            .map(|number| TableError::Row(create_row_key_error(&check, number as usize)))
            .collect())
    };

    let entries: Vec<(RowKeyCheck, KeyHashProbe)> =
        key_checks.into_iter().zip(key_probes).collect();
    for chunk in run_concurrent_chunks(inspect, entries, concurrency) {
        for check_errors in chunk {
            errors.extend(check_errors?);
        }

        if errors.len() >= max_errors {
            break;
        }
    }

    Ok(errors)
}

// Python's `iter_concurrent_chunks`, which fairspec-dataset keeps private: each chunk of
// up to `concurrency` entries runs on scoped threads, results in input order
fn run_concurrent_chunks<T, R, F>(
    func: F,
    items: Vec<T>,
    concurrency: Option<usize>,
) -> impl Iterator<Item = Vec<R>>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Sync,
{
    let size = concurrency
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from))
        .max(1);
    let mut entries = items.into_iter().peekable();
    std::iter::from_fn(move || {
        entries.peek()?;
        let chunk: Vec<T> = entries.by_ref().take(size).collect();
        if chunk.len() == 1 {
            return Some(chunk.into_iter().map(&func).collect());
        }
        let func = &func;
        Some(std::thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .into_iter()
                .map(|entry| scope.spawn(move || func(entry)))
                .collect();
            handles
                .into_iter()
                .map(|handle| {
                    handle
                        .join()
                        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                })
                .collect()
        }))
    })
}
