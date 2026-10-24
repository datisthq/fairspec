use std::collections::HashSet;

use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;
use crate::models::table_schema::TableSchema;

/// Check the key definitions of a table schema against its columns
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

fn create_error(message: String, json_pointer: String) -> FairspecError {
    FairspecError::Metadata(MetadataError {
        message,
        json_pointer,
        ..Default::default()
    })
}
