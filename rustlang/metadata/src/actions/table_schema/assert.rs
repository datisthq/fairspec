use super::validate::validate_table_schema;
use crate::models::descriptor::Descriptor;
use crate::models::exception::{Error, Result};
use crate::models::report::Report;
use crate::models::table_schema::TableSchema;

pub fn assert_table_schema(source: &Descriptor) -> Result<TableSchema> {
    let result = validate_table_schema(source, None)?;

    match result.table_schema {
        Some(table_schema) => Ok(table_schema),
        None => Err(Error::Invalid {
            message: "Invalid Table Schema".to_string(),
            report: Some(Report {
                valid: result.valid,
                errors: result.errors,
            }),
        }),
    }
}
