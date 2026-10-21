use fairspec_metadata::{ColumnProperty, TableSchema};
use indexmap::IndexMap;
use serde_json::{Map, Value};

use crate::plugins::ckan::models::field::CkanField;
use crate::plugins::ckan::models::schema::CkanSchema;

pub fn convert_table_schema_from_ckan(ckan_schema: &CkanSchema) -> TableSchema {
    let properties: IndexMap<String, ColumnProperty> =
        ckan_schema.fields.iter().map(convert_column).collect();

    TableSchema {
        properties: Some(properties),
        ..TableSchema::default()
    }
}

fn convert_column(ckan_field: &CkanField) -> (String, ColumnProperty) {
    let info = ckan_field.info.as_ref();

    let mut property = Map::new();
    if let Some(label) = info
        .and_then(|info| info.label.as_ref())
        .filter(|v| !v.is_empty())
    {
        property.insert("title".to_string(), Value::String(label.clone()));
    }
    if let Some(notes) = info
        .and_then(|info| info.notes.as_ref())
        .filter(|v| !v.is_empty())
    {
        property.insert("description".to_string(), Value::String(notes.clone()));
    }

    let column_type = info
        .and_then(|info| info.type_override.as_deref())
        .filter(|value| !value.is_empty())
        .or(Some(ckan_field.r#type.as_str()).filter(|value| !value.is_empty()))
        .unwrap_or("text")
        .to_lowercase();

    let (r#type, format) = match column_type.as_str() {
        "int" | "integer" => ("integer", None),
        "numeric" | "number" | "float" => ("number", None),
        "bool" | "boolean" => ("boolean", None),
        "date" => ("string", Some("date")),
        "time" => ("string", Some("local-time")),
        "timestamp" | "datetime" => ("string", Some("local-date-time")),
        "timestamptz" => ("string", Some("date-time")),
        "json" | "object" => ("object", None),
        "array" => ("array", None),
        _ => ("string", None),
    };
    property.insert("type".to_string(), Value::String(r#type.to_string()));
    if let Some(format) = format {
        property.insert("format".to_string(), Value::String(format.to_string()));
    }

    let property = serde_json::from_value(Value::Object(property))
        .expect("a CKAN column converts to a known property");
    (ckan_field.id.clone(), property)
}
