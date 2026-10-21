use super::load::load_table_schema;
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::table_schema::TableSchema;

pub fn resolve_table_schema(
    table_schema: Option<&PathOr<TableSchema>>,
) -> Result<Option<TableSchema>> {
    match table_schema {
        None => Ok(None),
        Some(PathOr::Path(path)) => load_table_schema(path).map(Some),
        Some(PathOr::Value(table_schema)) => Ok(Some(table_schema.clone())),
    }
}
