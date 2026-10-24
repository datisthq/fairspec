use std::fs;

use fairspec_metadata::Result;

use crate::helpers::cleanup::register_cleanup;

pub fn write_temp_file(
    content: impl AsRef<[u8]>,
    persist: bool,
    filename: Option<&str>,
    format: Option<&str>,
) -> Result<String> {
    let path = get_temp_file_path(persist, filename, format)?;
    fs::write(&path, content)?;
    Ok(path)
}

/// A vacant path in the system temp folder, deleted on exit unless `persist`.
pub fn get_temp_file_path(
    persist: bool,
    filename: Option<&str>,
    format: Option<&str>,
) -> Result<String> {
    let path = match filename.filter(|filename| !filename.is_empty()) {
        Some(filename) => tempfile::Builder::new()
            .prefix("tmp")
            .tempdir()?
            .keep()
            .join(filename),
        None => {
            let suffix = format
                .map(|format| format!(".{format}"))
                .unwrap_or_default();
            tempfile::Builder::new()
                .prefix("tmp")
                .suffix(&suffix)
                .tempfile()?
                .into_temp_path()
                .to_path_buf()
        }
    };

    if !persist {
        register_cleanup(path.clone());
    }
    Ok(format_path(&path))
}

pub(crate) fn format_path(path: &std::path::Path) -> String {
    let path = path.to_string_lossy();
    if cfg!(windows) {
        path.replace('\\', "/")
    } else {
        path.into_owned()
    }
}
