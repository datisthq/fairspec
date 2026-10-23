use serde_json::json;

use crate::_test::{run, write_file};

#[test]
fn test_should_preview_csv_file_as_json() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id,name\n1,[alice]\n");

    let result = run(&["file", "preview", &path, "--json"]);

    assert_eq!(result.code, 0, "{}", result.stderr);
    assert_eq!(
        result.json(),
        json!({"text": "id,name\n1,[alice]\n", "truncated": false})
    );
}

#[test]
fn test_should_limit_preview_to_lines() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.csv", "id\n1\n2\n3\n");

    let result = run(&["file", "preview", &path, "--lines", "2", "--json"]);

    assert_eq!(result.json(), json!({"text": "id\n1\n", "truncated": true}));
}

#[test]
fn test_should_limit_preview_to_sample_bytes() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(folder.path(), "table.txt", "abcdefgh");

    let result = run(&["file", "preview", &path, "--sample-bytes", "3", "--json"]);

    assert_eq!(result.json(), json!({"text": "abc", "truncated": true}));
}

#[test]
fn test_should_print_raw_source_with_brackets_intact() {
    let folder = tempfile::tempdir().unwrap();
    let path = write_file(
        folder.path(),
        "table.csv",
        "id,name\n1,[alice]\n2,[bold]bob[/bold]\n",
    );

    let result = run(&["file", "preview", &path]);

    assert_eq!(result.code, 0);
    assert!(
        result.stdout.contains("1,[alice]\n2,[bold]bob[/bold]\n"),
        "{}",
        result.stdout
    );
}
