use std::path::{Component, Path, PathBuf};

use crate::actions::file::temp::format_path;

/// Python's `os.path.abspath`: joined with the cwd and normalized lexically.
pub fn get_absolute_path(path: impl AsRef<Path>) -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_default();
    let mut absolute = PathBuf::new();
    for component in cwd.join(path).components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                absolute.pop();
            }
            component => absolute.push(component),
        }
    }
    absolute
}

/// Python's `safe_relpath`: cwd-relative with `/`, or absolute across Windows drives.
pub fn get_safe_relative_path(path: &Path) -> String {
    let path = get_absolute_path(path);
    let start = get_absolute_path(".");
    let parts: Vec<Component> = path.components().collect();
    let start_parts: Vec<Component> = start.components().collect();
    if parts.first() != start_parts.first() {
        return format_path(&path);
    }
    let common = parts
        .iter()
        .zip(&start_parts)
        .take_while(|(a, b)| a == b)
        .count();
    let relative: Vec<String> = std::iter::repeat_n("..".to_string(), start_parts.len() - common)
        .chain(
            parts[common..]
                .iter()
                .map(|part| part.as_os_str().to_string_lossy().into_owned()),
        )
        .collect();
    if relative.is_empty() {
        ".".to_string()
    } else {
        relative.join("/")
    }
}
