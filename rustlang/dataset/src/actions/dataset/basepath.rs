use std::path::{Component, PathBuf};

use fairspec_metadata::{Dataset, Error, Result, get_basepath, get_data_paths, get_is_remote_path};

use crate::helpers::path::{get_absolute_path, get_safe_relative_path};

/// The deepest local folder holding every data file, cwd-relative; `None` without local files.
pub fn get_dataset_basepath(dataset: &Dataset) -> Result<Option<String>> {
    let paths: Vec<String> = dataset
        .resources
        .iter()
        .flatten()
        .flat_map(get_data_paths)
        .collect();
    get_common_local_basepath(&paths)
}

pub fn get_common_local_basepath(paths: &[String]) -> Result<Option<String>> {
    let absolute_basepaths: Vec<PathBuf> = paths
        .iter()
        .filter(|path| !get_is_remote_path(path))
        .map(|path| get_absolute_path(get_basepath(path)))
        .collect();
    let Some((first, rest)) = absolute_basepaths.split_first() else {
        return Ok(None);
    };

    let mut common: Vec<Component> = first.components().collect();
    for path in rest {
        let length = common
            .iter()
            .zip(path.components())
            .take_while(|(a, b)| **a == *b)
            .count();
        common.truncate(length);
    }
    if common.is_empty() {
        return Err(Error::Path("Cannot find common basepath".to_string()));
    }

    let basepath = get_safe_relative_path(&common.iter().collect::<PathBuf>());
    Ok(Some(if basepath == "." {
        String::new()
    } else {
        basepath
    }))
}
