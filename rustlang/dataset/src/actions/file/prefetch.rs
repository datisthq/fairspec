use fairspec_metadata::{Resource, Result, get_data_paths, get_is_remote_path};

use super::copy::copy_file;
use super::temp::get_temp_file_path;
use crate::helpers::concurrency::run_concurrent_tasks;

pub fn prefetch_files(
    resource: &Resource,
    max_bytes: Option<usize>,
    concurrency: Option<usize>,
) -> Result<Vec<String>> {
    let paths = get_data_paths(resource);
    run_concurrent_tasks(
        |path: String| prefetch_file(&path, max_bytes),
        paths,
        concurrency,
    )
    .into_iter()
    .collect()
}

/// A local path to the file: the path itself, or a temp copy of a remote file.
pub fn prefetch_file(path: &str, max_bytes: Option<usize>) -> Result<String> {
    if !get_is_remote_path(path) {
        return Ok(path.to_string());
    }

    let new_path = get_temp_file_path(false, None, None)?;
    copy_file(path, &new_path, max_bytes)?;
    Ok(new_path)
}
