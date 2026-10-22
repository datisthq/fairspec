use fairspec_metadata::{
    Error, FileDialect, Resource, Result, TableSchema, get_data_first_path,
    get_supported_file_dialect,
};

use crate::plugins::sqlite::actions::database::connect::connect_database;
use crate::plugins::sqlite::actions::database::quote::quote_identifier;
use crate::plugins::sqlite::actions::table_schema::from_database::convert_table_schema_from_database;
use crate::plugins::sqlite::models::column::SqliteColumn;
use crate::plugins::sqlite::models::schema::SqliteSchema;

pub fn infer_table_schema_from_sqlite(resource: &Resource) -> Result<TableSchema> {
    let first_path = get_data_first_path(resource)
        .ok_or_else(|| Error::Load("Database is not defined".to_string()))?;

    let dialect = get_supported_file_dialect(resource, &["sqlite"])?
        .ok_or_else(|| Error::Load("Resource data is not compatible".to_string()))?;

    let connection = connect_database(&first_path, false)?;
    let to_error = |error: rusqlite::Error| Error::Load(error.to_string());

    let mut statement = connection
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .map_err(to_error)?;
    let tables: Vec<String> = statement
        .query_map([], |row| row.get(0))
        .map_err(to_error)?
        .collect::<rusqlite::Result<_>>()
        .map_err(to_error)?;

    let dialect_table_name = match &dialect {
        FileDialect::Sqlite(dialect) => dialect.table_name.clone(),
        _ => None,
    };
    let table_name = dialect_table_name
        .filter(|name| !name.is_empty())
        .or_else(|| tables.first().cloned())
        .filter(|name| !name.is_empty())
        .ok_or_else(|| Error::Load("Table name is not defined".to_string()))?;

    let mut statement = connection
        .prepare(&format!(
            "PRAGMA table_info({})",
            quote_identifier(&table_name)
        ))
        .map_err(to_error)?;
    let pragma_rows = statement
        .query_map([], |row| {
            let name: String = row.get("name")?;
            let data_type: Option<String> = row.get("type")?;
            let not_null: i64 = row.get("notnull")?;
            let default_value: Option<rusqlite::types::Value> = row.get("dflt_value")?;
            let primary_key: i64 = row.get("pk")?;
            Ok((name, data_type, not_null, default_value, primary_key))
        })
        .map_err(to_error)?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(to_error)?;

    let mut columns = Vec::new();
    let mut primary_key = Vec::new();
    for (name, data_type, not_null, default_value, pk) in pragma_rows {
        let data_type = data_type
            .filter(|data_type| !data_type.is_empty())
            .map_or_else(|| "text".to_string(), |data_type| data_type.to_lowercase());
        columns.push(SqliteColumn {
            is_nullable: not_null == 0,
            has_default_value: default_value.is_some(),
            ..SqliteColumn::new(name.clone(), data_type)
        });
        if pk != 0 {
            primary_key.push(name);
        }
    }

    let schema = SqliteSchema {
        name: table_name,
        columns,
        is_view: false,
        primary_key: (!primary_key.is_empty()).then_some(primary_key),
    };

    convert_table_schema_from_database(&schema)
}
