use std::path::Component;

use crate::actions::path::normalize::normalize_path;

#[test]
fn test_valid() {
    let depth = std::env::current_dir()
        .unwrap()
        .components()
        .filter(|component| matches!(component, Component::Normal(_)))
        .count();
    let absolute = format!("{}absolute/path/file.txt", "../".repeat(depth));
    let cases = [
        ("path/to/file.txt", None, "path/to/file.txt"),
        ("file.txt", Some("path/to"), "path/to/file.txt"),
        (
            "http://example.com/path/to/file.txt",
            None,
            "http://example.com/path/to/file.txt",
        ),
        (
            "http://example.com/path/to/file.txt?query=param",
            None,
            "http://example.com/path/to/file.txt?query=param",
        ),
        (
            "path/to/file.txt",
            Some("http://example.com"),
            "http://example.com/path/to/file.txt",
        ),
        ("file.txt", Some("/absolute/path"), absolute.as_str()),
        ("path/to/file.txt", Some(""), "path/to/file.txt"),
    ];
    for (path, basepath, expected) in cases {
        assert_eq!(
            normalize_path(path, basepath).unwrap(),
            expected,
            "{path} {basepath:?}"
        );
    }
}

#[test]
fn test_throw() {
    let cases = [
        ("/absolute/path/to/file.txt", None),
        ("../file.txt", Some("/folder")),
        ("../file.txt", Some("http://example.com/data")),
    ];
    for (path, basepath) in cases {
        assert!(
            normalize_path(path, basepath).is_err(),
            "{path} {basepath:?}"
        );
    }
}
