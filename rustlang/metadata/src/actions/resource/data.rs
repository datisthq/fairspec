use serde_json::Value;

use crate::models::data::ResourceDataPath;
use crate::models::resource::Resource;

pub fn get_data_path(resource: &Resource) -> Option<ResourceDataPath> {
    match resource.data.as_ref()? {
        Value::String(path) => Some(ResourceDataPath::Path(path.clone())),
        Value::Array(items) => items
            .iter()
            .map(|item| item.as_str().map(str::to_string))
            .collect::<Option<Vec<_>>>()
            .map(ResourceDataPath::Paths),
        _ => None,
    }
}

pub fn get_data_value(resource: &Resource) -> Option<&Value> {
    if get_data_path(resource).is_some_and(|path| !get_is_empty_path(&path)) {
        return None;
    }
    resource.data.as_ref()
}

pub fn get_data_records(resource: &Resource) -> Option<&[Value]> {
    match get_data_value(resource)? {
        Value::Array(records) if !records.is_empty() => Some(records),
        _ => None,
    }
}

pub fn get_data_paths(resource: &Resource) -> Vec<String> {
    match get_data_path(resource) {
        Some(ResourceDataPath::Path(path)) if !path.is_empty() => vec![path],
        Some(ResourceDataPath::Paths(paths)) => paths,
        _ => Vec::new(),
    }
}

pub fn get_data_first_path(resource: &Resource) -> Option<String> {
    match get_data_path(resource)? {
        ResourceDataPath::Path(path) => Some(path).filter(|path| !path.is_empty()),
        ResourceDataPath::Paths(paths) => paths.into_iter().next(),
    }
}

// Python tests the path for truthiness, so "" and [] count as no path
fn get_is_empty_path(path: &ResourceDataPath) -> bool {
    match path {
        ResourceDataPath::Path(path) => path.is_empty(),
        ResourceDataPath::Paths(paths) => paths.is_empty(),
    }
}
