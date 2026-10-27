use serde_json::{Map, Value};

use crate::actions::json::inspect::JsonIssue;
use crate::models::descriptor::Descriptor;

const META_SCHEMA_URL: &str = "https://json-schema.org/draft/2020-12/schema";
const PROFILE_PREFIX: &str = "https://fairspec.org/profiles/";
const PROFILE_SUFFIX: &str = "/data-schema.json";

const SCHEMA_MAP_KEYWORDS: [&str; 5] = [
    "$defs",
    "definitions",
    "properties",
    "patternProperties",
    "dependentSchemas",
];
const SCHEMA_KEYWORDS: [&str; 16] = [
    "additionalProperties",
    "unevaluatedProperties",
    "items",
    "additionalItems",
    "unevaluatedItems",
    "contains",
    "propertyNames",
    "not",
    "if",
    "then",
    "else",
    "contentSchema",
    "allOf",
    "anyOf",
    "oneOf",
    "prefixItems",
];

/// Tells the `$schema` a Data Schema may not name, and every `$ref` that does not
/// resolve within the document, which is never fetched.
pub fn inspect_data_schema(
    descriptor: &Descriptor,
    root_json_pointer: Option<&str>,
) -> Vec<JsonIssue> {
    let root_path = root_json_pointer.unwrap_or_default();
    let mut errors = Vec::new();

    if let Some(Value::String(schema)) = descriptor.get("$schema")
        && schema != META_SCHEMA_URL
        && !get_is_profile_url(schema)
    {
        errors.push(JsonIssue {
            message: "must be a Fairspec Data Schema profile or the JSON Schema 2020-12 dialect"
                .to_string(),
            json_pointer: format!("{root_path}/$schema"),
        });
        return errors;
    }

    let mut schemas = Vec::new();
    list_subschemas(descriptor, String::new(), &mut schemas);
    for (schema, pointer) in &schemas {
        for keyword in ["$ref", "$dynamicRef"] {
            if let Some(Value::String(reference)) = schema.get(keyword)
                && !get_is_local_ref(reference, descriptor, &schemas)
            {
                errors.push(JsonIssue {
                    message: "must resolve within the data schema".to_string(),
                    json_pointer: format!("{root_path}{pointer}/{keyword}"),
                });
            }
        }
    }

    errors
}

fn get_is_profile_url(url: &str) -> bool {
    url.strip_prefix(PROFILE_PREFIX)
        .and_then(|rest| rest.strip_suffix(PROFILE_SUFFIX))
        .is_some_and(|version| !version.is_empty() && !version.contains('/'))
}

fn list_subschemas<'a>(
    schema: &'a Map<String, Value>,
    pointer: String,
    result: &mut Vec<(&'a Map<String, Value>, String)>,
) {
    result.push((schema, pointer.clone()));
    for keyword in SCHEMA_MAP_KEYWORDS {
        let Some(Value::Object(map)) = schema.get(keyword) else {
            continue;
        };
        for (key, value) in map {
            if let Value::Object(item) = value {
                let token = key.replace('~', "~0").replace('/', "~1");
                list_subschemas(item, format!("{pointer}/{keyword}/{token}"), result);
            }
        }
    }
    for keyword in SCHEMA_KEYWORDS {
        match schema.get(keyword) {
            Some(Value::Array(items)) => {
                for (index, item) in items.iter().enumerate() {
                    if let Value::Object(item) = item {
                        list_subschemas(item, format!("{pointer}/{keyword}/{index}"), result);
                    }
                }
            }
            Some(Value::Object(item)) => {
                list_subschemas(item, format!("{pointer}/{keyword}"), result);
            }
            _ => {}
        }
    }
}

fn get_is_local_ref(
    reference: &str,
    root: &Map<String, Value>,
    schemas: &[(&Map<String, Value>, String)],
) -> bool {
    let (base, fragment) = reference.split_once('#').unwrap_or((reference, ""));

    let mut target = root;
    if !base.is_empty() {
        let found = schemas
            .iter()
            .find(|(schema, _)| schema.get("$id").and_then(Value::as_str) == Some(base));
        let Some((schema, _)) = found else {
            return false;
        };
        target = schema;
    }

    if fragment.is_empty() {
        return true;
    }
    if !fragment.starts_with('/') {
        return schemas.iter().any(|(schema, _)| {
            schema.get("$anchor").and_then(Value::as_str) == Some(fragment)
                || schema.get("$dynamicAnchor").and_then(Value::as_str) == Some(fragment)
        });
    }

    let Some(decoded) = decode_percent(fragment) else {
        return false;
    };

    let mut node: Option<&Value> = None;
    for (index, token) in decoded[1..].split('/').enumerate() {
        let key = token.replace("~1", "/").replace("~0", "~");
        let next = match (index, node) {
            (0, _) => target.get(&key),
            (_, Some(Value::Object(map))) => map.get(&key),
            (_, Some(Value::Array(items))) => get_array_item(items, &key),
            _ => None,
        };
        let Some(next) = next else {
            return false;
        };
        node = Some(next);
    }
    true
}

fn get_array_item<'a>(items: &'a [Value], key: &str) -> Option<&'a Value> {
    let is_index = key == "0"
        || (!key.starts_with('0') && !key.is_empty() && key.bytes().all(|b| b.is_ascii_digit()));
    if !is_index {
        return None;
    }
    key.parse::<usize>().ok().and_then(|index| items.get(index))
}

fn decode_percent(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hex = text.get(index + 1..index + 3)?;
            if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                return None;
            }
            decoded.push(u8::from_str_radix(hex, 16).ok()?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}
