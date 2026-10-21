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
        Some(PathOr::Value(file_dialect)) => Ok(Some(file_dialect.clone())),
    }
}
