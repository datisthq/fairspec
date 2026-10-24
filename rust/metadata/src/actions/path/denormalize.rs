use super::general::{
    UrlParts, format_path, get_absolute_path, get_is_remote_path, get_relative_path,
};
use super::normalize::create_error;
use crate::models::exception::Result;

pub fn denormalize_path(path: &str, basepath: Option<&str>) -> Result<String> {
    if get_is_remote_path(path) {
        return Ok(UrlParts::parse(path).format());
    }

    if let Some(basepath) = basepath.filter(|basepath| get_is_remote_path(basepath)) {
        let normalized_basepath = UrlParts::parse(basepath).format();
        if !path.starts_with(&normalized_basepath) {
            return Err(create_error(path, Some(basepath)));
        }
        let prefix = format!("{normalized_basepath}/");
        return Ok(path.strip_prefix(&prefix).unwrap_or(path).to_string());
    }

    let normalized_path = get_absolute_path(path);
    let normalized_basepath = get_absolute_path(basepath.unwrap_or_default());
    if !format_path(&normalized_path).starts_with(&format_path(&normalized_basepath)) {
        return Err(create_error(path, basepath));
    }

    get_relative_path(&normalized_path, &normalized_basepath)
        .ok_or_else(|| create_error(path, basepath))
}
