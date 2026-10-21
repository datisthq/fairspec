use super::general::{
    UrlParts, get_absolute_path, get_is_remote_path, get_relative_path, normalize_url_path,
    safe_relpath,
};
use crate::models::exception::{Error, Result};

pub fn normalize_path(path: &str, basepath: Option<&str>) -> Result<String> {
    if get_is_remote_path(path) {
        return Ok(UrlParts::parse(path).format());
    }

    if let Some(basepath) = basepath.filter(|basepath| get_is_remote_path(basepath)) {
        let mut url = UrlParts::parse(&format!("{basepath}/{path}"));
        url.path = normalize_url_path(&url.path);
        let normalized = url.format();
        if !normalized.starts_with(basepath) {
            return Err(create_error(path, Some(basepath)));
        }
        return Ok(normalized);
    }

    let basepath = basepath.unwrap_or_default();
    let joined = if basepath.is_empty() {
        path.to_string()
    } else {
        get_joined_path(basepath, path)
    };
    let relative = get_relative_path(&get_absolute_path(&joined), &get_absolute_path(basepath));
    if relative.is_none_or(|relative| relative.starts_with("..")) {
        return Err(create_error(path, Some(basepath)));
    }

    Ok(safe_relpath(&joined, None))
}

fn get_joined_path(basepath: &str, path: &str) -> String {
    std::path::Path::new(basepath)
        .join(path)
        .to_string_lossy()
        .into_owned()
}

pub(crate) fn create_error(path: &str, basepath: Option<&str>) -> Error {
    Error::Path(format!(
        "Path {path} is not a subpath of {}",
        basepath.unwrap_or("None")
    ))
}
