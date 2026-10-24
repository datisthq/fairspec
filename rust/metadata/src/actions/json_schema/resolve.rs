use super::load::load_json_schema;
use crate::models::exception::Result;
use crate::models::json_schema::JsonSchema;
use crate::models::path::PathOr;

pub fn resolve_json_schema(json_schema: Option<&PathOr<JsonSchema>>) -> Result<Option<JsonSchema>> {
    match json_schema {
        None => Ok(None),
        Some(PathOr::Path(path)) => load_json_schema(path, false).map(Some),
        Some(PathOr::Value(json_schema)) => Ok(Some(json_schema.clone())),
    }
}
