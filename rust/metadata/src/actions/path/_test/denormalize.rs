use crate::actions::path::denormalize::denormalize_path;

#[test]
fn test_denormalize_path() {
    let cases = [
        (
            "http://example.com/path/to/file.txt",
            None,
            "http://example.com/path/to/file.txt",
        ),
        (
            "http://example.com/path/to/file.txt",
            Some("data"),
            "http://example.com/path/to/file.txt",
        ),
        ("/tmp/data/file.csv", Some("/tmp"), "data/file.csv"),
        ("/tmp/file.csv", Some("/tmp"), "file.csv"),
        (
            "/tmp/data/nested/deep/file.csv",
            Some("/tmp/data/nested"),
            "deep/file.csv",
        ),
        (
            "/home/user/projects/data/file.csv",
            Some("/home/user/projects"),
            "data/file.csv",
        ),
    ];
    for (path, basepath, expected) in cases {
        assert_eq!(
            denormalize_path(path, basepath).unwrap(),
            expected,
            "{path}"
        );
    }
}

#[test]
fn test_denormalize_path_with_remote_basepath() {
    let path = denormalize_path("data/file.csv", Some("https://example.com")).unwrap_err();
    assert!(path.to_string().contains("is not a subpath"));
    assert_eq!(
        denormalize_path(
            "https://example.com/data/file.csv",
            Some("https://example.com")
        )
        .unwrap(),
        "https://example.com/data/file.csv"
    );
}
