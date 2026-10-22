use fairspec_metadata::{Result, TableSchema, get_column_properties, set_property_nullable};

use crate::plugins::sqlite::actions::column::from_database::convert_column_from_database;
use crate::plugins::sqlite::models::schema::SqliteSchema;

pub fn convert_table_schema_from_database(database_schema: &SqliteSchema) -> Result<TableSchema> {
    let mut columns = Vec::new();
    let mut required = Vec::new();

    for database_column in &database_schema.columns {
        let mut column = convert_column_from_database(database_column)?;

        if database_column.is_nullable {
            set_property_nullable(&mut column);
        } else {
            required.push(database_column.name.clone());
        }

        columns.push(column);
    }

    Ok(TableSchema {
        properties: Some(get_column_properties(&columns)),
        primary_key: database_schema.primary_key.clone(),
        required: Some(required),
        ..TableSchema::default()
    })
}
