use super::general::{UrlParts, get_absolute_path, get_is_remote_path, safe_relpath};
use crate::actions::descriptor::load::resolve_remote_url;
use crate::models::exception::Result;

pub fn get_basepath(path: &str) -> String {
    if get_is_remote_path(path) {
        let mut url = UrlParts::parse(path);
        if url.path.is_empty() {
            url.path = "/".to_string();
        }
        let url = url.format();
        let mut parts: Vec<&str> = url.split('/').collect();
        parts.pop();
        return parts.join("/");
    }

    let parent = get_absolute_path(path);
    let parent = parent.parent().unwrap_or(&parent);
    let relative = safe_relpath(&parent.to_string_lossy(), None);
    if relative == "." {
        String::new()
    } else {
        relative
    }
}

/// Like `get_basepath`, but follows the redirects of a remote path first.
pub fn resolve_basepath(path: &str) -> Result<String> {
    if !get_is_remote_path(path) {
        return Ok(get_basepath(path));
    }
    Ok(get_basepath(&resolve_remote_url(path)?))
}
