use serde_json::Value;

use crate::actions::path::normalize::normalize_path;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::resource::Resource;

pub fn normalize_resource(resource: &Resource, basepath: Option<&str>) -> Result<Descriptor> {
    let mut resource = resource.clone();
    map_resource_paths(&mut resource, |path| normalize_path(path, basepath))?;
    to_descriptor(&resource)
}

/// Rewrites every path of a resource: its data paths and the dialect and schema paths.
pub(crate) fn map_resource_paths(
    resource: &mut Resource,
    map_path: impl Fn(&str) -> Result<String>,
) -> Result<()> {
    match &mut resource.data {
        Some(Value::String(path)) => *path = map_path(path)?,
        Some(Value::Array(items)) => {
            for item in items {
                if let Value::String(path) = item {
                    *path = map_path(path)?;
                }
            }
        }
        _ => {}
    }
    if let Some(PathOr::Path(path)) = &mut resource.file_dialect {
        *path = map_path(path)?;
    }
    if let Some(PathOr::Path(path)) = &mut resource.data_schema {
        *path = map_path(path)?;
    }
    if let Some(PathOr::Path(path)) = &mut resource.table_schema {
        *path = map_path(path)?;
    }
    Ok(())
}

pub(crate) fn to_descriptor(resource: &Resource) -> Result<Descriptor> {
    match serde_json::to_value(resource) {
        Ok(Value::Object(descriptor)) => Ok(descriptor),
        Ok(_) => unreachable!("a resource serializes to an object"),
        Err(error) => Err(crate::models::exception::Error::Parse(error.to_string())),
    }
}
