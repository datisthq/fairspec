use std::collections::HashMap;
use std::sync::LazyLock;

use fairspec_metadata::{
    Descriptor, Error, PathOr, Resource, Result, denormalize_path, get_file_name,
    get_is_remote_path,
};
use regex::Regex;
use serde_json::Value;

static FILE_NAME_PARTS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(.*?)([^/]+?)(\.[^/]+(?:\.[^/]+)*)$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveFileProps {
    pub property_name: String,
    pub property_index: usize,
    pub normalized_path: String,
    pub denormalized_path: String,
}

/// Saves one file and returns the path the descriptor should reference.
pub type SaveFileCallback<'a> = dyn FnMut(&SaveFileProps) -> Result<String> + 'a;

/// Saves every file a resource references through `save_file` and returns the
/// descriptor pointing at the saved paths; a repeated target path gets a `-N` suffix.
pub fn save_resource_files(
    resource: &Resource,
    save_file: &mut SaveFileCallback,
    basepath: Option<&str>,
    with_remote: bool,
    without_folders: bool,
) -> Result<Descriptor> {
    let mut resource = resource.clone();
    let mut dedup_indexes: HashMap<String, usize> = HashMap::new();

    let mut save = |path: &str, name: &str, index: usize| -> Result<String> {
        let mut denormalized_path = denormalize_path(path, basepath)?;
        if get_is_remote_path(path) {
            if !with_remote {
                return Ok(path.to_string());
            }
            let Some(file_name) = get_file_name(path) else {
                return Ok(path.to_string());
            };
            denormalized_path = file_name;
        } else if without_folders {
            denormalized_path = denormalized_path.replace('/', "-");
        }

        let dedup_index = dedup_indexes.entry(denormalized_path.clone()).or_insert(0);
        let current = *dedup_index;
        *dedup_index += 1;
        if current > 0 {
            denormalized_path = FILE_NAME_PARTS
                .replace(&denormalized_path, format!("${{1}}${{2}}-{current}${{3}}"))
                .into_owned();
        }

        save_file(&SaveFileProps {
            property_name: name.to_string(),
            property_index: index,
            normalized_path: path.to_string(),
            denormalized_path,
        })
    };

    match &mut resource.data {
        Some(Value::String(path)) => *path = save(path, "data", 0)?,
        Some(Value::Array(items)) => {
            for (index, item) in items.iter_mut().enumerate() {
                if let Value::String(path) = item {
                    *path = save(path, "data", index)?;
                }
            }
        }
        _ => {}
    }
    if let Some(PathOr::Path(path)) = &mut resource.data_schema {
        *path = save(path, "dataSchema", 0)?;
    }
    if let Some(PathOr::Path(path)) = &mut resource.table_schema {
        *path = save(path, "tableSchema", 0)?;
    }

    match serde_json::to_value(&resource) {
        Ok(Value::Object(descriptor)) => Ok(descriptor),
        Ok(_) => Ok(Descriptor::new()),
        Err(error) => Err(Error::Parse(error.to_string())),
    }
}
