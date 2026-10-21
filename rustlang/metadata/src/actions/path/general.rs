use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use super::_shared::slugify::slugify_text;

static NON_SLUG_CHARS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^a-zA-Z0-9_]").unwrap());

// Python's `urllib.parse.uses_netloc`: these schemes always unparse with `//`
const NETLOC_SCHEMES: &[&str] = &[
    "",
    "ftp",
    "http",
    "gopher",
    "nntp",
    "telnet",
    "imap",
    "wais",
    "file",
    "mms",
    "https",
    "shttp",
    "snews",
    "prospero",
    "rtsp",
    "rtsps",
    "rtspu",
    "rsync",
    "svn",
    "svn+ssh",
    "sftp",
    "nfs",
    "git",
    "git+ssh",
    "ws",
    "wss",
    "itms-services",
];

/// A cwd-relative (or `start`-relative) path with `/` separators; an absolute
/// path when there is no relative one (another Windows drive).
pub fn safe_relpath(path: &str, start: Option<&str>) -> String {
    let path = get_absolute_path(path);
    let start = get_absolute_path(start.unwrap_or("."));
    get_relative_path(&path, &start).unwrap_or_else(|| format_path(&path))
}

pub fn get_file_protocol(path: &str) -> String {
    match split_url_scheme(path) {
        Some((scheme, _)) if scheme.len() >= 2 => scheme,
        _ => "file".to_string(),
    }
}

pub fn get_is_remote_path(path: &str) -> bool {
    get_file_protocol(path) != "file"
}

pub fn get_file_name(path: &str) -> Option<String> {
    let file_name = if get_is_remote_path(path) {
        let url = UrlParts::parse(path);
        url.path.rsplit('/').next().unwrap_or_default().to_string()
    } else {
        get_absolute_path(path)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default()
    };
    (!file_name.is_empty() && file_name.contains('.')).then_some(file_name)
}

pub fn get_file_extension(path: &str) -> Option<String> {
    let file_name = get_file_name(path)?;
    let extension = file_name.rsplit('.').next().unwrap_or_default();
    if file_name == format!(".{extension}") {
        return None;
    }
    Some(extension.to_string())
}

pub fn get_file_basename(path: &str) -> Option<String> {
    let file_name = get_file_name(path);
    match (get_file_extension(path), file_name) {
        (Some(extension), Some(file_name)) if !extension.is_empty() => {
            Some(file_name.replacen(&format!(".{extension}"), "", 1))
        }
        (_, file_name) => file_name,
    }
}

pub fn get_file_name_slug(path: &str) -> Option<String> {
    let basename = get_file_basename(path).filter(|basename| !basename.is_empty())?;
    let slug = slugify_text(&basename, "_");
    Some(NON_SLUG_CHARS.replace_all(&slug, "").into_owned())
}

/// Python's `urllib.parse.urlsplit`, with `params` left in `path`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UrlParts {
    pub scheme: String,
    pub netloc: String,
    pub path: String,
    pub query: String,
    pub fragment: String,
}

impl UrlParts {
    pub fn parse(url: &str) -> Self {
        let (scheme, rest) = split_url_scheme(url).unwrap_or_default();
        let rest = if scheme.is_empty() { url } else { rest };
        let (netloc, rest) = match rest.strip_prefix("//") {
            Some(rest) => {
                let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
                (rest[..end].to_string(), &rest[end..])
            }
            None => (String::new(), rest),
        };
        let (rest, fragment) = rest.split_once('#').unwrap_or((rest, ""));
        let (path, query) = rest.split_once('?').unwrap_or((rest, ""));
        Self {
            scheme,
            netloc,
            path: path.to_string(),
            query: query.to_string(),
            fragment: fragment.to_string(),
        }
    }

    /// Python 3.12's `urlunsplit`.
    pub fn format(&self) -> String {
        let mut url = self.path.clone();
        let uses_netloc = NETLOC_SCHEMES.contains(&self.scheme.as_str());
        if !self.netloc.is_empty() || (uses_netloc && !url.starts_with("//")) {
            if !url.is_empty() && !url.starts_with('/') {
                url.insert(0, '/');
            }
            url = format!("//{}{url}", self.netloc);
        }
        if !self.scheme.is_empty() {
            url = format!("{}:{url}", self.scheme);
        }
        if !self.query.is_empty() {
            url = format!("{url}?{}", self.query);
        }
        if !self.fragment.is_empty() {
            url = format!("{url}#{}", self.fragment);
        }
        url
    }
}

fn split_url_scheme(url: &str) -> Option<(String, &str)> {
    let (scheme, rest) = url.split_once(':')?;
    let mut chars = scheme.chars();
    let is_scheme = chars.next().is_some_and(|char| char.is_ascii_alphabetic())
        && chars.all(|char| char.is_ascii_alphanumeric() || "+-.".contains(char));
    is_scheme.then(|| (scheme.to_ascii_lowercase(), rest))
}

/// Python's `posixpath.normpath`, for URL paths.
pub(crate) fn normalize_url_path(path: &str) -> String {
    let slashes = path.len() - path.trim_start_matches('/').len();
    let prefix = match slashes {
        0 => "",
        2 => "//",
        _ => "/",
    };
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." if parts.last().is_some_and(|last| *last != "..") => {
                parts.pop();
            }
            ".." if prefix.is_empty() => parts.push(".."),
            ".." => {}
            part => parts.push(part),
        }
    }
    let normalized = format!("{prefix}{}", parts.join("/"));
    if normalized.is_empty() {
        ".".to_string()
    } else {
        normalized
    }
}

/// Python's `os.path.abspath`: joined with the cwd and normalized lexically.
pub(crate) fn get_absolute_path(path: &str) -> PathBuf {
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

/// Python's `os.path.relpath` on two absolute paths, `None` across drives.
pub(crate) fn get_relative_path(path: &Path, start: &Path) -> Option<String> {
    let path: Vec<Component> = path.components().collect();
    let start: Vec<Component> = start.components().collect();
    if path.first() != start.first() {
        return None;
    }
    let common = path.iter().zip(&start).take_while(|(a, b)| a == b).count();
    let parts: Vec<String> = std::iter::repeat_n("..".to_string(), start.len() - common)
        .chain(
            path[common..]
                .iter()
                .map(|part| part.as_os_str().to_string_lossy().into_owned()),
        )
        .collect();
    Some(if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    })
}

pub(crate) fn format_path(path: &Path) -> String {
    let path = path.to_string_lossy();
    if cfg!(windows) {
        path.replace('\\', "/")
    } else {
        path.into_owned()
    }
}
