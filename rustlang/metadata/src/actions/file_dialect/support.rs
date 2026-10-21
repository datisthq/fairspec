use serde_json::json;

use super::infer::infer_file_dialect_format;
use super::resolve::resolve_file_dialect;
use crate::actions::resource::data::get_data_paths;
use crate::models::exception::{Error, Result};
use crate::models::file_dialect::file_dialect::FileDialect;
use crate::models::resource::Resource;

pub fn get_supported_file_dialect(
    resource: &Resource,
    supported_formats: &[&str],
) -> Result<Option<FileDialect>> {
    if get_data_paths(resource).is_empty() {
        return Ok(None);
    }

    let mut resolved = resolve_file_dialect(resource.file_dialect.as_ref())?;
    if resolved.is_none()
        && let Some(format) = infer_file_dialect_format(resource)
    {
        let file_dialect = serde_json::from_value(json!({"format": format}))
            .map_err(|error| Error::Parse(error.to_string()))?;
        resolved = Some(file_dialect);
    }

    Ok(resolved.filter(|file_dialect| {
        file_dialect
            .format()
            .is_some_and(|format| supported_formats.contains(&format))
    }))
}
