use crate::actions::json_schema::load::load_json_schema;
use crate::models::data_schema::DataSchema;
use crate::models::exception::Result;
use crate::models::path::PathOr;

pub fn resolve_data_schema(data_schema: Option<&PathOr<DataSchema>>) -> Result<Option<DataSchema>> {
    match data_schema {
        None => Ok(None),
        Some(PathOr::Path(path)) => load_json_schema(path, false).map(Some),
        Some(PathOr::Value(data_schema)) => Ok(Some(data_schema.clone())),
    }
}
