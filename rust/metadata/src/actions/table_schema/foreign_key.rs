use crate::models::column::column::{Column, ColumnType};
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::MetadataError;
use crate::models::foreign_key::ForeignKey;
use crate::models::table_schema::TableSchema;

const NUMERIC_COLUMN_TYPES: [ColumnType; 3] =
    [ColumnType::Integer, ColumnType::Number, ColumnType::Decimal];

/// Check that each local column of a foreign key has the column type of the column it refers to
pub fn inspect_foreign_key_types(
    foreign_key: &ForeignKey,
    table_schema: &TableSchema,
    referenced_schema: &TableSchema,
    json_pointer: &str,
) -> Vec<FairspecError> {
    for (name, referenced_name) in foreign_key
        .columns
        .iter()
        .zip(&foreign_key.reference.columns)
    {
        let (Some(column_type), Some(referenced_type)) = (
            get_column_type(table_schema, name),
            get_column_type(referenced_schema, referenced_name),
        ) else {
            continue;
        };
        if column_type == referenced_type
            || (NUMERIC_COLUMN_TYPES.contains(&column_type)
                && NUMERIC_COLUMN_TYPES.contains(&referenced_type))
        {
            continue;
        }

        return vec![FairspecError::Metadata(MetadataError {
            message: format!(
                "foreign key column \"{name}\" is {}, but \"{referenced_name}\" is {}",
                format_column_type(column_type),
                format_column_type(referenced_type),
            ),
            json_pointer: json_pointer.to_string(),
            ..Default::default()
        })];
    }

    Vec::new()
}

fn get_column_type(table_schema: &TableSchema, name: &str) -> Option<ColumnType> {
    let property = table_schema.properties.as_ref()?.get(name)?;
    Some(Column::from_property(name, property.clone()).column_type())
}

fn format_column_type(column_type: ColumnType) -> String {
    serde_json::to_value(column_type)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_default()
}
