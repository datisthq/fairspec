use std::io::Cursor;

use fairspec_metadata::{
    Error, FileDialect, PathOr, Resource, Result, get_data_first_path, get_supported_file_dialect,
    resolve_table_schema,
};
use polars::prelude::*;
use rusqlite::types::ValueRef;
use serde_json::{Map, Value};

use crate::actions::table::normalize::normalize_table;
use crate::helpers::error::PolarsResultExt;
use crate::models::table::{LoadTableOptions, Table};
use crate::plugins::sqlite::actions::database::connect::connect_database;
use crate::plugins::sqlite::actions::database::quote::quote_identifier;
use crate::plugins::sqlite::actions::table_schema::infer::infer_table_schema_from_sqlite;

pub fn load_sqlite_table(resource: &Resource, options: &LoadTableOptions) -> Result<Table> {
    let Some(first_path) = get_data_first_path(resource) else {
        return Err(Error::Load("Resource path is not defined".to_string()));
    };
    let Some(file_dialect) = get_supported_file_dialect(resource, &["sqlite"])? else {
        return Err(Error::Load("Resource data is not compatible".to_string()));
    };

    let connection = connect_database(&first_path, false)?;
    let table_name = get_table_name(&connection, &file_dialect)?;
    let records = read_records(&connection, &table_name)?;

    let text = serde_json::to_vec(&records).map_err(|error| Error::Parse(error.to_string()))?;
    let mut table = JsonReader::new(Cursor::new(text))
        .infer_schema_len(None)
        .finish()
        .into_result()?
        .lazy();

    if options.denormalized != Some(true) {
        let table_schema = match resolve_table_schema(resource.table_schema.as_ref())? {
            Some(table_schema) => table_schema,
            None => infer_table_schema_from_sqlite(&Resource {
                data: Some(Value::String(first_path)),
                file_dialect: Some(PathOr::Value(file_dialect)),
                ..Resource::default()
            })?,
        };
        table = normalize_table(table, &table_schema)?;
    }

    Ok(table)
}

/// The dialect's table, or the first table by name.
pub(crate) fn get_table_name(
    connection: &rusqlite::Connection,
    file_dialect: &FileDialect,
) -> Result<String> {
    let dialect_table_name = match file_dialect {
        FileDialect::Sqlite(dialect) => dialect.table_name.clone(),
        _ => None,
    };
    if let Some(table_name) = dialect_table_name.filter(|name| !name.is_empty()) {
        return Ok(table_name);
    }

    let first_table: Option<String> = connection
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name",
            [],
            |row| row.get(0),
        )
        .ok();
    first_table
        .filter(|name| !name.is_empty())
        .ok_or_else(|| Error::Load("Table name is not defined".to_string()))
}

/// Rows as records; a blob reads as lossy UTF-8 text, where Python's twin keeps bytes.
fn read_records(
    connection: &rusqlite::Connection,
    table_name: &str,
) -> Result<Vec<Map<String, Value>>> {
    let to_error = |error: rusqlite::Error| Error::Load(error.to_string());
    let mut statement = connection
        .prepare(&format!("SELECT * FROM {}", quote_identifier(table_name)))
        .map_err(to_error)?;
    let names: Vec<String> = statement
        .column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();

    let mut rows = statement.query([]).map_err(to_error)?;
    let mut records = Vec::new();
    while let Some(row) = rows.next().map_err(to_error)? {
        let mut record = Map::new();
        for (index, name) in names.iter().enumerate() {
            let value = match row.get_ref(index).map_err(to_error)? {
                ValueRef::Null => Value::Null,
                ValueRef::Integer(value) => Value::from(value),
                ValueRef::Real(value) => Value::from(value),
                ValueRef::Text(text) => Value::String(String::from_utf8_lossy(text).into_owned()),
                ValueRef::Blob(blob) => Value::String(String::from_utf8_lossy(blob).into_owned()),
            };
            record.insert(name.clone(), value);
        }
        records.push(record);
    }
    Ok(records)
}
