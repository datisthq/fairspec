use crate::actions::path::general::get_file_extension;
use crate::models::descriptor::Descriptor;
use serde_json::{Value, json};

use crate::actions::file_dialect::from_frictionless::convert_file_dialect_from_frictionless;
use crate::actions::frictionless::mapping::FrictionlessRecorder;
use crate::actions::frictionless::name::convert_name_from_frictionless;
use crate::actions::frictionless::value::{get_non_empty_list, get_text};
use crate::actions::rights::from_frictionless::convert_rights_from_frictionless;
use crate::actions::table_schema::from_frictionless::convert_table_schema_from_frictionless;

const HASH_TYPES: [&str; 4] = ["md5", "sha1", "sha256", "sha512"];

/// Convert a Frictionless Data Resource to a Fairspec Resource descriptor
/// A `schema` or `dialect` path stays a path; loading it converts the file it names
pub fn convert_resource_from_frictionless(
    frictionless_resource: &Descriptor,
    record: Option<&mut FrictionlessRecorder>,
) -> Descriptor {
    let mut noop = FrictionlessRecorder::default();
    let record = record.unwrap_or(&mut noop);
    let mut resource = Descriptor::new();
    let path = frictionless_resource.get("path");

    if let Some(path) = path {
        resource.insert("data".to_string(), path.clone());
        record.record_verbatim("/data", "/path");
    } else if let Some(data) = frictionless_resource.get("data") {
        resource.insert("data".to_string(), convert_inline_data(data));
        record.record("/data", "/data");
    }

    if let Some(name) = get_text(frictionless_resource, "name") {
        resource.insert(
            "name".to_string(),
            convert_name_from_frictionless(name).into(),
        );
        record.record("/name", "/name");
    }

    if let Some(title) = get_text(frictionless_resource, "title") {
        resource.insert("titles".to_string(), json!([{"title": title}]));
        record.record("/titles", "/title");
    }

    if let Some(description) = get_text(frictionless_resource, "description") {
        resource.insert(
            "descriptions".to_string(),
            json!([{"description": description, "descriptionType": "Abstract"}]),
        );
        record.record("/descriptions", "/description");
    }

    if let Some(licenses) = get_non_empty_list(frictionless_resource, "licenses") {
        let rights_list = convert_rights_from_frictionless(
            licenses,
            Some(&mut record.nest("/rightsList", "/licenses")),
        );
        resource.insert("rightsList".to_string(), rights_list.into());
    }

    if let Some(bytes) = frictionless_resource.get("bytes").and_then(Value::as_f64) {
        resource.insert("sizes".to_string(), json!([format!("{bytes} bytes")]));
        record.record("/sizes", "/bytes");
    }

    if let Some(integrity) = get_text(frictionless_resource, "hash").and_then(convert_hash) {
        resource.insert("integrity".to_string(), integrity);
        record.record("/integrity", "/hash");
    }

    let dialect = frictionless_resource.get("dialect");
    if let Some(Value::String(dialect)) = dialect {
        resource.insert("fileDialect".to_string(), dialect.clone().into());
        record.record("/fileDialect", "/dialect");
    } else if let Some(path) = path {
        let first_path = match path {
            Value::Array(paths) => paths.first().and_then(Value::as_str),
            _ => path.as_str(),
        };
        let format = match frictionless_resource.get("format").and_then(Value::as_str) {
            Some(format) => format.to_string(),
            None => get_media_format(frictionless_resource)
                .map(String::from)
                .or_else(|| {
                    first_path
                        .filter(|path| !path.is_empty())
                        .and_then(get_file_extension)
                })
                .unwrap_or_default(),
        };
        let dialect = dialect.and_then(Value::as_object);
        let file_dialect = convert_file_dialect_from_frictionless(
            dialect,
            Some(&format),
            Some(&mut record.nest("/fileDialect", "/dialect")),
        );
        if let Some(file_dialect) = file_dialect {
            resource.insert("fileDialect".to_string(), file_dialect.into());
            let source = match dialect {
                Some(_) => "/dialect",
                None => "/format",
            };
            record.record("/fileDialect", source);
            record.record("/fileDialect/format", "/format");
        }
    }

    match frictionless_resource.get("schema") {
        Some(Value::String(schema)) => {
            resource.insert("tableSchema".to_string(), schema.clone().into());
            record.record("/tableSchema", "/schema");
        }
        Some(Value::Object(schema)) => {
            let table_schema = convert_table_schema_from_frictionless(
                schema,
                Some(&mut record.nest("/tableSchema", "/schema")),
            );
            resource.insert("tableSchema".to_string(), table_schema.into());
            record.record("/tableSchema", "/schema");
        }
        _ => {}
    }

    resource
}

fn get_media_format(frictionless_resource: &Descriptor) -> Option<&'static str> {
    match frictionless_resource
        .get("mediatype")
        .and_then(Value::as_str)?
    {
        "text/csv" => Some("csv"),
        "text/tab-separated-values" => Some("tsv"),
        "application/json" => Some("json"),
        _ => None,
    }
}

fn convert_inline_data(data: &Value) -> Value {
    let Some(rows) = data.as_array() else {
        return data.clone();
    };
    if !rows.iter().all(Value::is_array) {
        return data.clone();
    }

    let empty = Vec::new();
    let header = rows.first().and_then(Value::as_array).unwrap_or(&empty);
    let records = rows
        .iter()
        .skip(1)
        .filter_map(Value::as_array)
        .map(|row| {
            let record: Descriptor = header
                .iter()
                .enumerate()
                .map(|(index, name)| {
                    let value = row.get(index).cloned().unwrap_or(Value::Null);
                    (stringify_name(name), value)
                })
                .collect();
            Value::Object(record)
        })
        .collect();
    Value::Array(records)
}

// JavaScript's `String(name)`
fn stringify_name(name: &Value) -> String {
    match name {
        Value::String(name) => name.clone(),
        other => other.to_string(),
    }
}

fn convert_hash(hash: &str) -> Option<Value> {
    let (r#type, value) = match hash.contains(':') {
        true => {
            let mut parts = hash.split(':');
            (parts.next()?, parts.next()?)
        }
        false => ("md5", hash),
    };
    if r#type.is_empty() || value.is_empty() || !HASH_TYPES.contains(&r#type) {
        return None;
    }
    Some(json!({"type": r#type, "hash": value}))
}
