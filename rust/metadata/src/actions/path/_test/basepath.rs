use crate::actions::path::basepath::get_basepath;

#[test]
fn test_get_basepath() {
    let cases = [
        (
            "http://example.com/path/to/file.txt",
            "http://example.com/path/to",
        ),
        (
            "https://example.com/path/to/file.txt",
            "https://example.com/path/to",
        ),
        (
            "https://example.com/path/to/file.txt?query=param",
            "https://example.com/path/to",
        ),
        (
            "https://example.com/path/to/file.txt#section",
            "https://example.com/path/to",
        ),
        (
            "https://example.com/path/to/",
            "https://example.com/path/to",
        ),
        ("https://example.com", "https://example.com"),
        ("some/path/to/file.txt", "some/path/to"),
        ("some/path/to/", "some/path"),
        ("file.txt", ""),
    ];
    for (path, expected) in cases {
        assert_eq!(get_basepath(path), expected, "{path}");
    }
}
