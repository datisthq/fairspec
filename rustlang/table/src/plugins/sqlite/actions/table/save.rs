use fairspec_metadata::{Error, PathOr, Resource, Result, get_supported_file_dialect};
use polars::prelude::*;
use rusqlite::types::Value as SqlValue;
use serde_json::Value;

use super::load::get_table_name;
use crate::actions::table::denormalize::denormalize_table;
use crate::actions::table_schema::infer::infer_table_schema_from_table;
use crate::helpers::error::PolarsResultExt;
use crate::models::column::DenormalizeColumnOptions;
use crate::models::schema::InferTableSchemaOptions;
use crate::models::table::{SaveTableOptions, Table};
use crate::plugins::sqlite::actions::database::connect::connect_database;
use crate::plugins::sqlite::actions::database::quote::quote_identifier;
use crate::plugins::sqlite::actions::table_schema::to_database::convert_table_schema_to_database;
use crate::plugins::sqlite::models::schema::SqliteSchema;
use crate::plugins::sqlite::settings::NATIVE_TYPES;

const BUFFER_SIZE: usize = 10_000;

pub fn save_sqlite_table(table: Table, options: &SaveTableOptions) -> Result<String> {
    let path = &options.path;
    let resource = Resource {
        data: Some(Value::String(path.clone())),
        file_dialect: options.file_dialect.clone().map(PathOr::Value),
        ..Resource::default()
    };
    let Some(file_dialect) = get_supported_file_dialect(&resource, &["sqlite"])? else {
        return Err(Error::Load("Saving options is not compatible".to_string()));
    };

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

    let mut connection = connect_database(path, true)?;
    let table_name = get_table_name(&connection, &file_dialect)?;
    let sqlite_schema = convert_table_schema_to_database(&table_schema, &table_name)?;
    define_table(&connection, &sqlite_schema, options.overwrite == Some(true))?;
    populate_table(&mut connection, &table_name, table)?;

    Ok(path.clone())
}

fn define_table(
    connection: &rusqlite::Connection,
    schema: &SqliteSchema,
    overwrite: bool,
) -> Result<()> {
    let to_error = |error: rusqlite::Error| Error::Load(error.to_string());
    if overwrite {
        connection
            .execute(
                &format!("DROP TABLE IF EXISTS {}", quote_identifier(&schema.name)),
                [],
            )
            .map_err(to_error)?;
    }

    let mut columns: Vec<String> = schema
        .columns
        .iter()
        .map(|column| {
            let null = if column.is_nullable { "" } else { " NOT NULL" };
            format!(
                "{} {}{null}",
                quote_identifier(&column.name),
                column.data_type
            )
        })
        .collect();
    if let Some(primary_key) = schema.primary_key.as_ref().filter(|key| !key.is_empty()) {
        let names: Vec<String> = primary_key
            .iter()
            .map(|name| quote_identifier(name))
            .collect();
        columns.push(format!("PRIMARY KEY ({})", names.join(", ")));
    }

    let sql = format!(
        "CREATE TABLE {} ({})",
        quote_identifier(&schema.name),
        columns.join(", ")
    );
    connection.execute(&sql, []).map_err(to_error)?;
    Ok(())
}

fn populate_table(
    connection: &mut rusqlite::Connection,
    table_name: &str,
    table: Table,
) -> Result<()> {
    let to_error = |error: rusqlite::Error| Error::Load(error.to_string());
    let frame = table.collect().into_result()?;
    if frame.height() == 0 {
        return Ok(());
    }

    let names: Vec<String> = frame
        .get_column_names()
        .iter()
        .map(|name| quote_identifier(name))
        .collect();
    let placeholders = vec!["?"; names.len()].join(", ");
    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({placeholders})",
        quote_identifier(table_name),
        names.join(", ")
    );

    for start in (0..frame.height()).step_by(BUFFER_SIZE) {
        let transaction = connection.transaction().map_err(to_error)?;
        {
            let mut statement = transaction.prepare(&sql).map_err(to_error)?;
            for index in start..frame.height().min(start + BUFFER_SIZE) {
                let values = frame
                    .columns()
                    .iter()
                    .map(|column| column.get(index).map(convert_sql_value))
                    .collect::<PolarsResult<Vec<_>>>()
                    .into_result()?;
                statement
                    .execute(rusqlite::params_from_iter(values))
                    .map_err(to_error)?;
            }
        }
        transaction.commit().map_err(to_error)?;
    }
    Ok(())
}

/// The value Python's sqlite3 binds for what `frame.to_dicts()` holds.
fn convert_sql_value(value: AnyValue) -> SqlValue {
    match value {
        AnyValue::Null => SqlValue::Null,
        AnyValue::Boolean(value) => SqlValue::Integer(value as i64),
        AnyValue::String(value) => SqlValue::Text(value.to_string()),
        AnyValue::StringOwned(value) => SqlValue::Text(value.to_string()),
        AnyValue::Float32(value) => SqlValue::Real(value as f64),
        AnyValue::Float64(value) => SqlValue::Real(value),
        value if value.is_integer() => value
            .extract::<i64>()
            .map_or(SqlValue::Null, SqlValue::Integer),
        value => SqlValue::Text(value.to_string()),
    }
}
