use fairspec_metadata::{Result, TableSchema, get_columns};
use serde_json::Value;

use crate::plugins::sqlite::actions::column::to_database::convert_column_to_database;
use crate::plugins::sqlite::models::schema::SqliteSchema;

pub fn convert_table_schema_to_database(
    table_schema: &TableSchema,
    table_name: &str,
) -> Result<SqliteSchema> {
    let descriptor = match serde_json::to_value(table_schema) {
        Ok(Value::Object(descriptor)) => descriptor,
        _ => Default::default(),
    };

    let columns = get_columns(&descriptor)?
        .iter()
        .map(|column| {
            let is_nullable = column
                .nullable()
                .unwrap_or_else(|| !column.required().unwrap_or_default());
            convert_column_to_database(column, is_nullable)
        })
        .collect();

    Ok(SqliteSchema {
        name: table_name.to_string(),
        columns,
        is_view: false,
        primary_key: table_schema
            .primary_key
            .clone()
            .filter(|key| !key.is_empty()),
    })
}
