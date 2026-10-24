use fairspec_metadata::{Column, Descriptor, Result, create_column_from_property};
use serde_json::Value;

use crate::plugins::sqlite::models::column::SqliteColumn;

pub fn convert_column_from_database(database_column: &SqliteColumn) -> Result<Column> {
    let mut property = Descriptor::new();
    if let Some(r#type) = convert_type(&database_column.data_type) {
        property.insert("type".to_string(), Value::from(r#type));
    }
    if let Some(comment) = database_column
        .comment
        .as_ref()
        .filter(|comment| !comment.is_empty())
    {
        property.insert("description".to_string(), Value::from(comment.as_str()));
    }

    create_column_from_property(&database_column.name, &property)
}

fn convert_type(database_type: &str) -> Option<&'static str> {
    match database_type.to_lowercase().as_str() {
        "blob" | "text" => Some("string"),
        "integer" => Some("integer"),
        "numeric" | "real" => Some("number"),
        _ => None,
    }
}
