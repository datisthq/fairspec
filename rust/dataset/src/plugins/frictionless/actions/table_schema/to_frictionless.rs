use fairspec_metadata::{Column, Descriptor, Error, Result, get_base_property_type, get_columns};
use serde_json::{Value, json};

use crate::plugins::frictionless::helpers::value::{copy_value, get_is_truthy, get_text};

/// Convert a Fairspec Table Schema to a Frictionless Table Schema descriptor
pub fn convert_table_schema_to_frictionless(table_schema: &Descriptor) -> Result<Descriptor> {
    let mut schema = Descriptor::new();

    let fields = get_columns(table_schema)?
        .iter()
        .map(convert_column)
        .collect::<Result<Vec<_>>>()?;
    schema.insert("fields".to_string(), fields.into());

    // Fairspec formats read an empty cell as null, which Frictionless does only through `""`
    let mut missing_values = table_schema
        .get("missingValues")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if !missing_values.contains(&json!("")) {
        missing_values.push(json!(""));
    }
    if missing_values.len() != 1 {
        schema.insert("missingValues".to_string(), missing_values.into());
    }

    copy_value(&mut schema, "primaryKey", table_schema.get("primaryKey"));
    copy_value(&mut schema, "uniqueKeys", table_schema.get("uniqueKeys"));

    if let Some(foreign_keys) = table_schema.get("foreignKeys").and_then(Value::as_array) {
        let foreign_keys: Vec<Value> = foreign_keys
            .iter()
            .map(|foreign_key| {
                let reference = &foreign_key["reference"];
                json!({
                    "fields": foreign_key["columns"],
                    "reference": {
                        "resource": reference.get("resource").unwrap_or(&json!("")),
                        "fields": reference["columns"],
                    },
                })
            })
            .collect();
        schema.insert("foreignKeys".to_string(), foreign_keys.into());
    }

    Ok(schema)
}

fn convert_column(column: &Column) -> Result<Value> {
    let column_type = match serde_json::to_value(column.column_type()) {
        Ok(Value::String(column_type)) => column_type,
        _ => return Err(Error::Parse("Column type is not a string".to_string())),
    };
    let property = match serde_json::to_value(column.property()) {
        Ok(Value::Object(property)) => property,
        Ok(_) => Descriptor::new(),
        Err(error) => return Err(Error::Parse(error.to_string())),
    };

    let mut field = Descriptor::new();
    field.insert("name".to_string(), column.name().into());
    field.extend(convert_type(&column_type, &property));
    let mut constraints = Descriptor::new();

    for key in ["title", "description", "rdfType"] {
        if let Some(text) = get_text(&property, key) {
            field.insert(key.to_string(), text.into());
        }
    }

    if column.nullable() != Some(true) && column_type != "unknown" {
        constraints.insert("required".to_string(), Value::Bool(true));
    }

    match column_type.as_str() {
        "integer" | "number" | "decimal" => {
            for key in ["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"] {
                copy_value(&mut constraints, key, property.get(key));
            }
            copy_truthy(&mut field, "groupChar", &property, "groupChar");
            if get_is_truthy(property.get("withText")) {
                field.insert("bareNumber".to_string(), Value::Bool(false));
            }
            if column_type != "integer" {
                copy_truthy(&mut field, "decimalChar", &property, "decimalChar");
            }
        }
        "string" => {
            copy_value(&mut constraints, "minLength", property.get("minLength"));
            copy_value(&mut constraints, "maxLength", property.get("maxLength"));
            copy_truthy(&mut constraints, "pattern", &property, "pattern");
        }
        "boolean" => {
            copy_truthy(&mut field, "trueValues", &property, "trueValues");
            copy_truthy(&mut field, "falseValues", &property, "falseValues");
        }
        "list" => {
            copy_truthy(&mut field, "delimiter", &property, "delimiter");
            if let Some(item_type) = property.get("itemType").and_then(Value::as_str) {
                copy_value(
                    &mut field,
                    "itemType",
                    get_item_type(item_type).map(Value::from).as_ref(),
                );
            }
            copy_value(&mut constraints, "minLength", property.get("minItems"));
            copy_value(&mut constraints, "maxLength", property.get("maxItems"));
        }
        "categorical" => {
            copy_value(&mut field, "categories", property.get("categories"));
            if get_is_truthy(property.get("withOrder")) {
                field.insert("categoriesOrdered".to_string(), Value::Bool(true));
            }
        }
        "date" | "time" | "local-time" | "date-time" | "local-date-time" => {
            copy_truthy(&mut field, "format", &property, "temporalFormat");
        }
        _ => {}
    }

    if let Some(Value::Array(values)) = property.get("enum") {
        let values: Vec<Value> = values
            .iter()
            .filter(|value| !value.is_null())
            .cloned()
            .collect();
        constraints.insert("enum".to_string(), values.into());
    }

    if !constraints.is_empty() {
        field.insert("constraints".to_string(), constraints.into());
    }

    Ok(Value::Object(field))
}

fn convert_type(column_type: &str, property: &Descriptor) -> Descriptor {
    let field = match column_type {
        "string" | "hex" | "wkt" | "wkb" => json!({"type": "string"}),
        "email" => json!({"type": "string", "format": "email"}),
        "url" => json!({"type": "string", "format": "uri"}),
        "base64" => json!({"type": "string", "format": "binary"}),
        "categorical" => {
            let is_integer = get_base_property_type(property.get("type")) == Some("integer");
            json!({"type": if is_integer { "integer" } else { "string" }})
        }
        "integer" => json!({"type": "integer"}),
        "number" | "decimal" => json!({"type": "number"}),
        "boolean" => json!({"type": "boolean"}),
        "date" => json!({"type": "date"}),
        "time" | "local-time" => json!({"type": "time"}),
        "date-time" | "local-date-time" => json!({"type": "datetime"}),
        "duration" => json!({"type": "duration"}),
        "list" => json!({"type": "list"}),
        "array" => json!({"type": "array"}),
        "object" => json!({"type": "object"}),
        "geojson" => json!({"type": "geojson"}),
        "topojson" => json!({"type": "geojson", "format": "topojson"}),
        _ => json!({"type": "any"}),
    };
    match field {
        Value::Object(field) => field,
        _ => Descriptor::new(),
    }
}

fn get_item_type(item_type: &str) -> Option<&'static str> {
    match item_type {
        "string" => Some("string"),
        "integer" => Some("integer"),
        "number" => Some("number"),
        "boolean" => Some("boolean"),
        "date" => Some("date"),
        "time" | "local-time" => Some("time"),
        "date-time" | "local-date-time" => Some("datetime"),
        _ => None,
    }
}

fn copy_truthy(target: &mut Descriptor, to: &str, source: &Descriptor, from: &str) {
    if let Some(value) = source.get(from).filter(|value| get_is_truthy(Some(value))) {
        target.insert(to.to_string(), value.clone());
    }
}
