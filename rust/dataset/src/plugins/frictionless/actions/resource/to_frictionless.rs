use fairspec_metadata::{Descriptor, Error, Resource, Result, infer_resource_name};
use serde_json::{Value, json};

use crate::plugins::frictionless::actions::file_dialect::to_frictionless::convert_file_dialect_to_frictionless;
use crate::plugins::frictionless::actions::rights::to_frictionless::convert_rights_to_frictionless;
use crate::plugins::frictionless::actions::table_schema::to_frictionless::convert_table_schema_to_frictionless;
use crate::plugins::frictionless::helpers::value::{get_non_empty_list, get_object, get_text};

/// Convert a denormalized Fairspec Resource to a Frictionless Data Resource descriptor
/// Its `tableSchema` and `fileDialect` must already be objects, not paths
pub fn convert_resource_to_frictionless(
    resource: &Descriptor,
    resource_number: usize,
) -> Result<Descriptor> {
    let mut frictionless_resource = Descriptor::new();

    let name = match resource.get("name").and_then(Value::as_str) {
        Some(name) => name.to_string(),
        None => {
            let typed: Resource = serde_json::from_value(Value::Object(resource.clone()))
                .map_err(|error| Error::Parse(error.to_string()))?;
            infer_resource_name(&typed, Some(resource_number))
        }
    };
    frictionless_resource.insert("name".to_string(), name.into());

    match resource.get("data") {
        Some(data) if data.is_string() || get_is_path_list(data) => {
            frictionless_resource.insert("path".to_string(), data.clone());
        }
        Some(data) => {
            frictionless_resource.insert("data".to_string(), data.clone());
        }
        None => {}
    }

    let table_schema = get_object(resource, "tableSchema");
    if table_schema.is_some() {
        frictionless_resource.insert("type".to_string(), "table".into());
    }

    if let Some(title) = get_first_text(resource, "titles", "title") {
        frictionless_resource.insert("title".to_string(), title.into());
    }

    if let Some(description) = get_first_text(resource, "descriptions", "description") {
        frictionless_resource.insert("description".to_string(), description.into());
    }

    if let Some(file_dialect) = get_object(resource, "fileDialect")
        && let Some((format, dialect)) = convert_file_dialect_to_frictionless(file_dialect)
    {
        frictionless_resource.insert("format".to_string(), format.into());
        if !dialect.is_empty() {
            frictionless_resource.insert("dialect".to_string(), dialect.into());
        }
    }

    if let Some(integrity) = get_object(resource, "integrity") {
        let r#type = integrity
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let hash = integrity
            .get("hash")
            .and_then(Value::as_str)
            .unwrap_or_default();
        frictionless_resource.insert("hash".to_string(), format!("{type}:{hash}").into());
    }

    let bytes = resource
        .get("sizes")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .find_map(get_size_bytes);
    if let Some(bytes) = bytes {
        frictionless_resource.insert("bytes".to_string(), bytes);
    }

    if let Some(rights_list) = get_non_empty_list(resource, "rightsList") {
        let licenses = convert_rights_to_frictionless(rights_list);
        frictionless_resource.insert("licenses".to_string(), licenses.into());
    }

    if let Some(table_schema) = table_schema {
        let schema = convert_table_schema_to_frictionless(table_schema)?;
        frictionless_resource.insert("schema".to_string(), schema.into());
    }

    Ok(frictionless_resource)
}

pub(crate) fn get_first_text<'a>(
    descriptor: &'a Descriptor,
    list: &str,
    key: &str,
) -> Option<&'a str> {
    descriptor
        .get(list)
        .and_then(Value::as_array)
        .and_then(|items| items.first())
        .and_then(Value::as_object)
        .and_then(|item| get_text(item, key))
}

fn get_is_path_list(data: &Value) -> bool {
    data.as_array()
        .is_some_and(|items| !items.is_empty() && items.iter().all(Value::is_string))
}

// JavaScript's `Number` of the digits in a `<n> bytes` size
fn get_size_bytes(size: &str) -> Option<Value> {
    let digits = size.strip_suffix(" bytes")?;
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    match digits.parse::<u64>() {
        Ok(bytes) => Some(json!(bytes)),
        Err(_) => digits.parse::<f64>().ok().map(|bytes| json!(bytes)),
    }
}
