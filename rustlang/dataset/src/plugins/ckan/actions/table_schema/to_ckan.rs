use fairspec_metadata::{Column, Result, TableSchema, get_columns};
use serde_json::Value;

use crate::plugins::ckan::models::field::{CkanField, CkanFieldInfo};
use crate::plugins::ckan::models::schema::CkanSchema;

pub fn convert_table_schema_to_ckan(table_schema: &TableSchema) -> Result<CkanSchema> {
    let descriptor = match serde_json::to_value(table_schema) {
        Ok(Value::Object(descriptor)) => descriptor,
        _ => Default::default(),
    };

    let fields = get_columns(&descriptor)?
        .iter()
        .map(convert_column)
        .collect();

    Ok(CkanSchema { fields })
}

fn convert_column(column: &Column) -> CkanField {
    let property = serde_json::to_value(column.property()).unwrap_or_default();
    let get_text = |key: &str| {
        property
            .get(key)
            .and_then(Value::as_str)
            .filter(|value| !value.is_empty())
            .map(String::from)
    };
    let title = get_text("title");
    let description = get_text("description");

    let info = (title.is_some() || description.is_some()).then(|| CkanFieldInfo {
        label: title,
        notes: description,
        type_override: Some(convert_type(column).to_string()),
    });

    CkanField {
        id: column.name().to_string(),
        r#type: convert_type(column).to_string(),
        info,
    }
}

fn convert_type(column: &Column) -> &'static str {
    match column {
        Column::String(_) => "text",
        Column::Integer(_) => "int",
        Column::Number(_) => "numeric",
        Column::Boolean(_) => "bool",
        Column::Date(_) => "date",
        Column::Time(_) | Column::LocalTime(_) => "time",
        Column::DateTime(_) => "timestamptz",
        Column::LocalDateTime(_) => "timestamp",
        Column::Object(_) => "json",
        Column::Array(_) => "array",
        _ => "text",
    }
}
