use fairspec_metadata::Descriptor;
use serde_json::{Value, json};

use crate::plugins::frictionless::helpers::list::convert_field_names;
use crate::plugins::frictionless::helpers::name::convert_name_from_frictionless;
use crate::plugins::frictionless::helpers::value::{
    copy_value, get_is_truthy, get_list, get_non_empty_list, get_object, get_text,
};

const STRING_KINDS: [&str; 6] = [
    "string",
    "email",
    "url",
    "base64",
    "categorical",
    "yearmonth",
];
const NUMERIC_KINDS: [&str; 3] = ["integer", "number", "year"];

/// Convert a Frictionless Table Schema to a Fairspec Table Schema descriptor
pub fn convert_table_schema_from_frictionless(schema: &Descriptor) -> Descriptor {
    let mut table_schema = Descriptor::new();
    let mut properties = Descriptor::new();
    let mut unique_keys = get_list(schema, "uniqueKeys").cloned().unwrap_or_default();

    for field in get_list(schema, "fields").into_iter().flatten() {
        let Some(field) = field.as_object() else {
            continue;
        };
        let name = field
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_default();
        properties.insert(name.to_string(), Value::Object(convert_field(field)));
        let constraints = get_object(field, "constraints");
        if get_is_truthy(constraints.and_then(|constraints| constraints.get("unique"))) {
            unique_keys.push(json!([name]));
        }
    }

    table_schema.insert("properties".to_string(), Value::Object(properties));

    // Fairspec formats already read an empty cell as null, Frictionless's default `[""]`
    if let Some(missing_values) = get_non_empty_list(schema, "missingValues") {
        let is_default = missing_values.len() == 1 && missing_values[0] == json!("");
        if !is_default {
            table_schema.insert("missingValues".to_string(), missing_values.clone().into());
        }
    }

    if let Some(primary_key) = schema.get("primaryKey") {
        let primary_key = convert_field_names(primary_key);
        if primary_key.as_array().is_some_and(|keys| !keys.is_empty()) {
            table_schema.insert("primaryKey".to_string(), primary_key);
        }
    }

    if !unique_keys.is_empty() {
        table_schema.insert("uniqueKeys".to_string(), unique_keys.into());
    }

    if let Some(foreign_keys) = get_non_empty_list(schema, "foreignKeys") {
        let foreign_keys: Vec<Value> = foreign_keys
            .iter()
            .filter_map(Value::as_object)
            .map(|foreign_key| {
                let mut reference = Descriptor::new();
                let source = get_object(foreign_key, "reference");
                if let Some(resource) = source.and_then(|source| get_text(source, "resource")) {
                    reference.insert(
                        "resource".to_string(),
                        convert_name_from_frictionless(resource).into(),
                    );
                }
                let fields = source.and_then(|source| source.get("fields"));
                reference.insert(
                    "columns".to_string(),
                    fields.map(convert_field_names).unwrap_or(Value::Null),
                );
                let columns = foreign_key.get("fields").map(convert_field_names);
                json!({"columns": columns, "reference": reference})
            })
            .collect();
        table_schema.insert("foreignKeys".to_string(), foreign_keys.into());
    }

    table_schema
}

fn convert_field(field: &Descriptor) -> Descriptor {
    let empty = Descriptor::new();
    let constraints = get_object(field, "constraints").unwrap_or(&empty);
    let (kind, mut property) = convert_type(field);

    for key in ["title", "description", "rdfType"] {
        if let Some(text) = get_text(field, key) {
            property.insert(key.to_string(), text.into());
        }
    }
    if let Some(example) = field.get("example") {
        property.insert("examples".to_string(), json!([example]));
    }
    copy_value(
        &mut property,
        "missingValues",
        get_non_empty_list(field, "missingValues").map(|_| &field["missingValues"]),
    );

    copy_value(
        &mut property,
        "enum",
        get_list(constraints, "enum").map(|_| &constraints["enum"]),
    );

    if NUMERIC_KINDS.contains(&kind) {
        for key in ["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"] {
            copy_value(
                &mut property,
                key,
                constraints.get(key).filter(|value| value.is_number()),
            );
        }
    }

    if STRING_KINDS.contains(&kind) {
        copy_value(&mut property, "minLength", constraints.get("minLength"));
        copy_value(&mut property, "maxLength", constraints.get("maxLength"));
        if let Some(pattern) = get_text(constraints, "pattern") {
            property.insert("pattern".to_string(), pattern.into());
        }
    }

    if kind == "list" {
        copy_value(&mut property, "minItems", constraints.get("minLength"));
        copy_value(&mut property, "maxItems", constraints.get("maxLength"));
    }

    if (kind == "object" || kind == "array")
        && let Some(json_schema) = get_object(constraints, "jsonSchema")
    {
        for (key, value) in json_schema {
            property.insert(key.clone(), value.clone());
        }
    }

    if !get_is_truthy(constraints.get("required"))
        && let Some(Value::String(r#type)) = property.get("type")
    {
        let r#type = json!([r#type, "null"]);
        property.insert("type".to_string(), r#type);
    }

    property
}

fn convert_type(field: &Descriptor) -> (&'static str, Descriptor) {
    let format = field.get("format").and_then(Value::as_str);
    let temporal_format = convert_temporal_format(format);
    let has_categories = get_list(field, "categories").is_some();
    let field_type = field
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("string");

    let (kind, property) = match field_type {
        "string" if has_categories => ("categorical", convert_categorical("string", field)),
        "string" => match format {
            Some("email") => ("email", json!({"type": "string", "format": "email"})),
            Some("uri") => ("url", json!({"type": "string", "format": "url"})),
            Some("binary") => ("base64", json!({"type": "string", "format": "base64"})),
            _ => ("string", json!({"type": "string"})),
        },
        "integer" if has_categories => ("categorical", convert_categorical("integer", field)),
        "integer" => {
            let mut property = json!({"type": "integer"});
            copy_text(&mut property, field, "groupChar", "groupChar");
            set_with_text(&mut property, field);
            ("integer", property)
        }
        "number" => {
            let mut property = json!({"type": "number"});
            copy_text(&mut property, field, "decimalChar", "decimalChar");
            copy_text(&mut property, field, "groupChar", "groupChar");
            set_with_text(&mut property, field);
            ("number", property)
        }
        "boolean" => {
            let mut property = json!({"type": "boolean"});
            for key in ["trueValues", "falseValues"] {
                if let Some(values) = field.get(key).filter(|value| value.is_array()) {
                    property[key] = values.clone();
                }
            }
            ("boolean", property)
        }
        "date" => ("date", create_temporal("date", temporal_format)),
        "time" => {
            let with_offset = temporal_format.is_some_and(|format| format.contains('z'));
            let format = if with_offset { "time" } else { "local-time" };
            ("time", create_temporal(format, temporal_format))
        }
        "datetime" => {
            let is_local = temporal_format.is_some_and(|format| !format.contains('z'));
            let format = if is_local {
                "local-date-time"
            } else {
                "date-time"
            };
            ("datetime", create_temporal(format, temporal_format))
        }
        "year" => ("year", json!({"type": "integer"})),
        "yearmonth" => (
            "yearmonth",
            json!({"type": "string", "pattern": "^\\d{4}-\\d{2}$"}),
        ),
        "duration" => ("duration", json!({"type": "string", "format": "duration"})),
        "geopoint" => match format {
            Some("array") => ("array", json!({"type": "array"})),
            Some("object") => ("object", json!({"type": "object"})),
            _ => ("string", json!({"type": "string"})),
        },
        "geojson" => {
            let format = if format == Some("topojson") {
                "topojson"
            } else {
                "geojson"
            };
            ("geojson", json!({"type": "object", "format": format}))
        }
        "object" => ("object", json!({"type": "object"})),
        "array" => ("array", json!({"type": "array"})),
        "list" => {
            let mut property = json!({"type": "string", "format": "list"});
            copy_text(&mut property, field, "delimiter", "delimiter");
            let item_type = field
                .get("itemType")
                .and_then(Value::as_str)
                .and_then(get_item_type);
            if let Some(item_type) = item_type {
                property["itemType"] = item_type.into();
            }
            ("list", property)
        }
        _ => ("unknown", json!({})),
    };

    match property {
        Value::Object(property) => (kind, property),
        _ => (kind, Descriptor::new()),
    }
}

fn get_item_type(item_type: &str) -> Option<&'static str> {
    match item_type {
        "string" => Some("string"),
        "integer" => Some("integer"),
        "number" => Some("number"),
        "boolean" => Some("boolean"),
        "date" => Some("date"),
        "time" => Some("local-time"),
        "datetime" => Some("date-time"),
        _ => None,
    }
}

fn convert_categorical(r#type: &str, field: &Descriptor) -> Value {
    let mut property =
        json!({"type": r#type, "format": "categorical", "categories": field["categories"]});
    if get_is_truthy(field.get("categoriesOrdered")) {
        property["withOrder"] = Value::Bool(true);
    }
    property
}

fn create_temporal(format: &str, temporal_format: Option<&str>) -> Value {
    let mut property = json!({"type": "string", "format": format});
    if let Some(temporal_format) = temporal_format.filter(|format| !format.is_empty()) {
        property["temporalFormat"] = temporal_format.into();
    }
    property
}

fn convert_temporal_format(format: Option<&str>) -> Option<&str> {
    match format {
        None | Some("" | "default" | "any") => None,
        Some(format) => Some(format.strip_prefix("fmt:").unwrap_or(format)),
    }
}

fn copy_text(property: &mut Value, field: &Descriptor, from: &str, to: &str) {
    if let Some(text) = get_text(field, from) {
        property[to] = text.into();
    }
}

fn set_with_text(property: &mut Value, field: &Descriptor) {
    if field.get("bareNumber") == Some(&Value::Bool(false)) {
        property["withText"] = Value::Bool(true);
    }
}
