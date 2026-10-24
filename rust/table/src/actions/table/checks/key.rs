use std::sync::{Arc, Mutex, PoisonError};

use fairspec_metadata::{Error, Result, RowError, RowPrimaryKeyError, RowUniqueKeyError};
use polars::prelude::*;

use crate::helpers::error::PolarsResultExt;
use crate::models::schema::SchemaMapping;

#[derive(Debug, Clone)]
pub struct RowKeyCheck {
    pub key_columns: Vec<String>,
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
        checks.push(create_row_key_check(primary_key, KeyType::Primary)?);
    }

    for key in mapping
        .target
        .unique_keys
        .iter()
        .flatten()
        .filter(is_present)
    {
        checks.push(create_row_key_check(key, KeyType::Unique)?);
    }

    Ok(checks)
}

/// Records the hash of every present key while another query reads the table.
#[derive(Clone)]
pub struct KeyHashProbe {
    pub flag_expr: Expr,
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
    let keys = &check.key_columns;

    // Polars calls this per batch; it returns all-false, so the flag joins the reading
    // query's filter without changing its rows
    let recorder = Arc::clone(&hashes);
    let record_hashes = move |batch: Column| {
        let size = batch.len();
        recorder
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(batch.take_materialized_series());
        Ok(BooleanChunked::full(PlSmallStr::EMPTY, false, size).into_column())
    };

    let key_expr = if keys.len() == 1 {
        col(keys[0].as_str())
    } else {
        as_struct(keys.iter().map(|name| col(name.as_str())).collect())
    };
    // Hashed inside polars, so a batch keeps 8 bytes per key, not the key values
    let flag_expr = when(create_key_presence_expr(keys))
        .then(key_expr.hash(0))
        .otherwise(lit(NULL))
        .map(record_hashes, |_, field| {
            Ok(Field::new(field.name().clone(), DataType::Boolean))
        });

    KeyHashProbe { flag_expr, hashes }
}

/// A key with any null value is never a duplicate.
pub fn create_key_presence_expr(key_columns: &[String]) -> Expr {
    let exprs: Vec<Expr> = key_columns
        .iter()
        .map(|name| col(name.as_str()).is_not_null())
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

fn create_row_key_check(key_columns: &[String], key_type: KeyType) -> Result<RowKeyCheck> {
    let Some(first_column) = key_columns.first() else {
        return Err(Error::Load(
            "A key must name at least one column".to_string(),
        ));
    };
    // The running count of each key's rows, which polars has without is_first_distinct
    let partition: Vec<Expr> = key_columns.iter().map(|name| col(name.as_str())).collect();
    let is_error_expr = col(first_column.as_str())
        .is_null()
        .cum_count(false)
        .over(partition)
        .into_result()?
        .gt(lit(1))
        .and(create_key_presence_expr(key_columns));

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
        is_error_expr,
        error_template,
    })
}
