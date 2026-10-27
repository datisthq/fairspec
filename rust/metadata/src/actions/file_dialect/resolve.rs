use super::assert::assert_file_dialect;
use super::load::load_file_dialect;
use crate::models::exception::Result;
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::path::PathOr;

pub fn resolve_file_dialect(
    file_dialect: Option<&PathOr<FileDialect>>,
) -> Result<Option<FileDialect>> {
    match file_dialect {
        None => Ok(None),
        Some(PathOr::Path(path)) => load_file_dialect(path).map(Some),
        // A dialect that is not valid never reads a file
        Some(PathOr::Value(file_dialect)) => {
            let descriptor = serde_json::to_value(file_dialect)
                .ok()
                .and_then(|value| match value {
                    serde_json::Value::Object(object) => Some(object),
                    _ => None,
                })
                .unwrap_or_default();
            assert_file_dialect(&descriptor)?;
            Ok(Some(file_dialect.clone()))
        }
    }
}
