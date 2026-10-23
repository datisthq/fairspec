use fairspec_library::fairspec_metadata::{PathOr, Resource};
use serde_json::Value;

use crate::helpers::file_dialect::get_file_dialect;
use crate::helpers::resource::select_resource;
use crate::models::output::Exit;
use crate::params::common::SelectParams;
use crate::params::file_dialect::DialectParams;
use crate::session::Session;

/// The table a command reads: the path with its dialect and schema, or a dataset's resource.
pub fn create_table_resource(
    session: &Session,
    path: Option<&str>,
    select: &SelectParams,
    dialect: &DialectParams,
    table_schema: Option<&str>,
) -> Result<Resource, Exit> {
    match path.filter(|path| !path.is_empty()) {
        Some(path) => Ok(Resource {
            data: Some(Value::String(path.to_string())),
            file_dialect: get_file_dialect(path, dialect),
            table_schema: table_schema.map(|schema| PathOr::Path(schema.to_string())),
            ..Resource::default()
        }),
        None => select_resource(
            session,
            select.dataset.as_deref(),
            select.resource.as_deref(),
        ),
    }
}
