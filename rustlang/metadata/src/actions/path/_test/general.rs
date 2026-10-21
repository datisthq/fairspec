use crate::actions::path::general::{
    get_file_extension, get_file_name, get_file_name_slug, get_is_remote_path,
};

#[test]
fn test_get_is_remote_path() {
    let cases = [
        ("http://example.com/path/to/file.txt", true),
        ("https://example.com/path/to/file.txt", true),
        ("ftp://example.com/path/to/file.txt", true),
        ("file:///path/to/file.txt", false),
        ("/path/to/file.txt", false),
        ("path/to/file.txt", false),
        ("./file.txt", false),
        ("../file.txt", false),
        ("", false),
        ("http:example.com", true),
        ("C:\\data\\file.csv", false),
        ("C:/data/file.csv", false),
    ];
    for (path, expected) in cases {
        assert_eq!(get_is_remote_path(path), expected, "{path}");
    }
}

#[test]
fn test_get_file_name() {
    let cases = [
        ("file.txt", Some("file.txt")),
        ("some/path/to/file.txt", Some("file.txt")),
        ("http://example.com/path/to/file.txt", Some("file.txt")),
        ("https://example.com/path/to/file.txt", Some("file.txt")),
        (
            "https://example.com/path/to/file.txt?query=param",
            Some("file.txt"),
        ),
        (
            "https://example.com/path/to/file.txt#section",
            Some("file.txt"),
        ),
        (
            "https://example.com/path/to/file.txt?query=param#section",
            Some("file.txt"),
        ),
        ("https://example.com/path/", None),
        ("some/path/", None),
    ];
    for (path, expected) in cases {
        assert_eq!(get_file_name(path).as_deref(), expected, "{path}");
    }
}

#[test]
fn test_get_file_extension() {
    let cases = [
        ("/data/users.csv", Some("csv")),
        ("https://example.com/data/products.json", Some("json")),
        ("/data/file.CSV", Some("CSV")),
        ("/data/file.tar.gz", Some("gz")),
        ("/data/file", None),
        ("/data/folder/", None),
        ("/data/file.backup.csv", Some("csv")),
        ("/data/.gitignore", None),
        ("https://example.com/file.json?key=value", Some("json")),
        ("https://example.com/file.pdf#page=1", Some("pdf")),
    ];
    for (path, expected) in cases {
        assert_eq!(get_file_extension(path).as_deref(), expected, "{path}");
    }
}

#[test]
fn test_get_file_name_slug() {
    let cases = [
        ("/data/users.csv", Some("users")),
        ("https://example.com/data/products.json", Some("products")),
        ("/data/folder/", None),
        ("/data/file.backup.csv", Some("file_backup")),
        ("/data/My Data File!.csv", Some("my_data_file")),
        ("/data/données.csv", Some("donnees")),
        ("/data/Straße.csv", Some("strasse")),
        ("/data/данные.csv", Some("dannye")),
        ("/data/post2024Data.csv", Some("post2024_data")),
        ("", None),
        ("document.txt", Some("document")),
        ("https://example.com/file.json?key=value", Some("file")),
        ("https://example.com/report.pdf#page=1", Some("report")),
        ("/data/.gitignore", Some("gitignore")),
        ("/data/MyDocument.PDF", Some("my_document")),
        ("/data/my-file-name.csv", Some("my_file_name")),
    ];
    for (path, expected) in cases {
        assert_eq!(get_file_name_slug(path).as_deref(), expected, "{path}");
    }
}
