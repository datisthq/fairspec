use std::collections::HashSet;

use serde_json::Value;

use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;
use crate::models::table_schema::TableSchema;

const NULLABLE_KEYWORDS: [&str; 4] = ["enum", "const", "default", "examples"];

/// Check the required columns, patterns and key definitions of a table schema against its columns
pub fn inspect_table_schema(
    table_schema: &TableSchema,
    root_json_pointer: Option<&str>,
) -> Vec<FairspecError> {
    let root = match root_json_pointer {
        None | Some("/") => "",
        Some(root) => root,
    };
    let names: HashSet<&str> = table_schema
        .properties
        .iter()
        .flat_map(|properties| properties.keys().map(String::as_str))
        .collect();
    let mut errors = Vec::new();

    let check_columns = |columns: &[String], pointer: &str, errors: &mut Vec<FairspecError>| {
        for (index, column) in columns.iter().enumerate() {
            if !names.contains(column.as_str()) {
                errors.push(create_error(
                    format!("column \"{column}\" is not in properties"),
                    format!("{root}{pointer}/{index}"),
                ));
            }
        }
    };

    check_columns(
        table_schema.required.as_deref().unwrap_or_default(),
        "/required",
        &mut errors,
    );
    for (name, property) in table_schema.properties.iter().flatten() {
        let property = serde_json::to_value(property).unwrap_or_default();
        let Some(pattern) = property.get("pattern").and_then(Value::as_str) else {
            continue;
        };
        if !jsonschema_regex::is_valid_ecma_regex(pattern) {
            errors.push(create_error(
                format!("pattern \"{pattern}\" is not a valid regular expression"),
                format!("{root}/properties/{}/pattern", escape_json_pointer(name)),
            ));
        }
    }
    check_columns(
        table_schema.primary_key.as_deref().unwrap_or_default(),
        "/primaryKey",
        &mut errors,
    );
    for (index, unique_key) in table_schema.unique_keys.iter().flatten().enumerate() {
        check_columns(unique_key, &format!("/uniqueKeys/{index}"), &mut errors);
    }
    for (index, foreign_key) in table_schema.foreign_keys.iter().flatten().enumerate() {
        check_columns(
            &foreign_key.columns,
            &format!("/foreignKeys/{index}/columns"),
            &mut errors,
        );
        if foreign_key.columns.len() != foreign_key.reference.columns.len() {
            errors.push(create_error(
                "foreign key columns and reference columns differ in length".to_string(),
                format!("{root}/foreignKeys/{index}/reference/columns"),
            ));
        }
    }

    errors
}

/// Check that enum, const, default and examples hold null only where the column type allows it
pub fn inspect_table_schema_nulls(
    descriptor: &Descriptor,
    root_json_pointer: Option<&str>,
) -> Vec<FairspecError> {
    let root = match root_json_pointer {
        None | Some("/") => "",
        Some(root) => root,
    };
    let mut errors = Vec::new();
    let Some(properties) = descriptor.get("properties").and_then(Value::as_object) else {
        return errors;
    };

    for (name, property) in properties {
        let Some(property) = property.as_object() else {
            continue;
        };
        match property.get("type") {
            None | Some(Value::Null) => continue,
            Some(Value::Array(types)) if types.iter().any(|item| item == "null") => continue,
            _ => {}
        }

        let pointer = format!("{root}/properties/{}", escape_json_pointer(name));
        let message =
            format!("null is not allowed: the type of column \"{name}\" does not include \"null\"");
        for keyword in NULLABLE_KEYWORDS {
            match (keyword, property.get(keyword)) {
                ("enum" | "examples", Some(Value::Array(items))) => {
                    for (index, item) in items.iter().enumerate() {
                        if item.is_null() {
                            errors.push(create_error(
                                message.clone(),
                                format!("{pointer}/{keyword}/{index}"),
                            ));
                        }
                    }
                }
                ("const" | "default", Some(Value::Null)) => {
                    errors.push(create_error(
                        message.clone(),
                        format!("{pointer}/{keyword}"),
                    ));
                }
                _ => {}
            }
        }
    }

    errors
}

fn escape_json_pointer(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

fn create_error(message: String, json_pointer: String) -> FairspecError {
    FairspecError::Metadata(MetadataError {
        message,
        json_pointer,
        ..Default::default()
    })
}
