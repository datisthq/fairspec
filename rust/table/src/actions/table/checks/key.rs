use std::sync::{Arc, Mutex, PoisonError};

use fairspec_metadata::{
    Error, Result, RowError, RowPrimaryKeyError, RowUniqueKeyError, get_columns,
};
use polars::prelude::*;

use crate::actions::column::inspect::FlagFactory;
use crate::actions::column::normalize::normalize_column;
use crate::actions::table::_shared::helpers::merge_missing_values;
use crate::actions::table::normalize::get_schema_descriptor;
use crate::helpers::error::PolarsResultExt;
use crate::models::column::ColumnMapping;
use crate::models::schema::SchemaMapping;

#[derive(Debug, Clone)]
pub struct RowKeyCheck {
    pub key_columns: Vec<String>,
    pub key_exprs: Vec<Expr>,
    pub is_error_expr: Expr,
    pub error_template: RowError,
}

pub fn create_row_key_checks(mapping: &SchemaMapping) -> Result<Vec<RowKeyCheck>> {
    let is_present = |key: &&Vec<String>| {
        key.iter().all(|name| {
            mapping
                .source
                .columns
                .iter()
                .any(|column| &column.name == name)
        })
    };
    let mut checks = Vec::new();

    if let Some(primary_key) = mapping
        .target
        .primary_key
        .as_ref()
        .filter(|k| !k.is_empty())
        .filter(is_present)
    {
        checks.push(create_row_key_check(
            mapping,
            primary_key,
            KeyType::Primary,
        )?);
    }

    for key in mapping
        .target
        .unique_keys
        .iter()
        .flatten()
        .filter(is_present)
    {
        checks.push(create_row_key_check(mapping, key, KeyType::Unique)?);
    }

    Ok(checks)
}

/// Records the hash of every present key while another query reads the table.
#[derive(Clone)]
pub struct KeyHashProbe {
    pub create_flag_expr: FlagFactory,
    pub hashes: Arc<Mutex<Vec<Series>>>,
}

impl KeyHashProbe {
    pub fn get_has_duplicate_key(&self) -> bool {
        let hashes = self.hashes.lock().unwrap_or_else(PoisonError::into_inner);
        let Some((first, rest)) = hashes.split_first() else {
            return false;
        };
        let mut all = first.clone();
        for batch in rest {
            if all.append(batch).is_err() {
                return true;
            }
        }
        let all = all.drop_nulls();
        all.n_unique().map_or(true, |count| count < all.len())
    }
}

pub fn create_key_hash_probe(check: &RowKeyCheck) -> KeyHashProbe {
    let hashes: Arc<Mutex<Vec<Series>>> = Arc::default();
    let key_columns = check.key_columns.clone();
    let key_exprs = check.key_exprs.clone();

    // Polars calls this per batch; it returns all-false, so the flag joins the reading
    // query's filter without changing its rows
    let recorder = Arc::clone(&hashes);
    let record_hashes = Arc::new(move |batch: Column| {
        let size = batch.len();
        recorder
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(batch.take_materialized_series());
        Ok(BooleanChunked::full(PlSmallStr::EMPTY, false, size).into_column())
    });

    // A key column the reading query already parses is read from it: the streaming
    // engine shares no repeated expression, so parsing it again costs a second parse
    let create_flag_expr: FlagFactory =
        Arc::new(move |read_typed: &dyn Fn(&str) -> Option<Expr>| {
            let keys: Vec<Expr> = key_columns
                .iter()
                .zip(&key_exprs)
                .map(|(name, key)| read_typed(name).unwrap_or_else(|| key.clone()))
                .collect();
            let key_expr = if keys.len() == 1 {
                keys[0].clone()
            } else {
                as_struct(keys.clone())
            };
            // polars passes a constant batch, such as a parsed all-`true` key, as one row;
            // adding the raw cell's null flag, 0 wherever the key is present, keeps every row
            let row_expr = col(key_columns[0].as_str())
                .is_null()
                .cast(DataType::UInt64);
            let record_hashes = Arc::clone(&record_hashes);
            // Hashed inside polars, so a batch keeps 8 bytes per key, not the key values
            when(create_key_presence_expr(&keys))
                .then(key_expr.hash(0) + row_expr)
                .otherwise(lit(NULL))
                .map(
                    move |batch| record_hashes(batch),
                    |_, field| Ok(Field::new(field.name().clone(), DataType::Boolean)),
                )
        });

    KeyHashProbe {
        create_flag_expr,
        hashes,
    }
}

/// A key with any null value is never a duplicate.
pub fn create_key_presence_expr(key_exprs: &[Expr]) -> Expr {
    let exprs: Vec<Expr> = key_exprs
        .iter()
        .map(|expr| expr.clone().is_not_null())
        .collect();
    exprs
        .into_iter()
        .reduce(|left, right| left.and(right))
        .unwrap_or_else(|| lit(false))
}

/// The template error for a row, its number filled in.
pub fn create_row_key_error(check: &RowKeyCheck, row_number: usize) -> RowError {
    let mut error = check.error_template.clone();
    match &mut error {
        RowError::PrimaryKey(error) => error.row_number = row_number,
        RowError::UniqueKey(error) => error.row_number = row_number,
        RowError::Check(error) => error.row_number = row_number,
    }
    error
}

enum KeyType {
    Primary,
    Unique,
}

fn create_row_key_check(
    mapping: &SchemaMapping,
    key_columns: &[String],
    key_type: KeyType,
) -> Result<RowKeyCheck> {
    let Some(first_column) = key_columns.first() else {
        return Err(Error::Load(
            "A key must name at least one column".to_string(),
        ));
    };
    let key_exprs = key_columns
        .iter()
        .map(|name| create_key_column_expr(mapping, name))
        .collect::<Result<Vec<Expr>>>()?;
    // The running count of each key's rows, which polars has without is_first_distinct
    let is_error_expr = col(first_column.as_str())
        .is_null()
        .cum_count(false)
        .over(key_exprs.clone())
        .into_result()?
        .gt(lit(1))
        .and(create_key_presence_expr(&key_exprs));

    let column_names = key_columns.to_vec();
    let error_template = match key_type {
        KeyType::Primary => RowError::PrimaryKey(RowPrimaryKeyError {
            column_names,
            ..Default::default()
        }),
        KeyType::Unique => RowError::UniqueKey(RowUniqueKeyError {
            column_names,
            ..Default::default()
        }),
    };

    Ok(RowKeyCheck {
        key_columns: key_columns.to_vec(),
        key_exprs,
        is_error_expr,
        error_template,
    })
}

// Keys compare the typed values, so `1.0` and `1` in a number column are one key
fn create_key_column_expr(mapping: &SchemaMapping, name: &str) -> Result<Expr> {
    let source = mapping
        .source
        .columns
        .iter()
        .find(|column| column.name == name);
    let columns = get_columns(&get_schema_descriptor(&mapping.target))?;
    let column = columns.iter().find(|column| column.name() == name);
    let (Some(source), Some(column)) = (source, column) else {
        return Ok(col(name));
    };

    let target = merge_missing_values(column, &mapping.target);
    let column_mapping = ColumnMapping {
        source: source.clone(),
        target,
    };
    Ok(normalize_column(&column_mapping, false))
}
