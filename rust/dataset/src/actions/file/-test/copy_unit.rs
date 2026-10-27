use std::fs;

use crate::actions::file::copy::copy_file;
use crate::actions::file::temp::write_temp_file;

fn copy_to(content: impl AsRef<[u8]>, target: &std::path::Path) -> Vec<u8> {
    let source = write_temp_file(content, false, None, None).unwrap();
    copy_file(&source, target.to_str().unwrap(), None).unwrap();
    fs::read(target).unwrap()
}

#[test]
fn test_copies_file() {
    let tmp_path = tempfile::tempdir().unwrap();
    let target = tmp_path.path().join("target.txt");
    let data = copy_to("test content", &target);
    assert!(target.exists());
    assert_eq!(data, b"test content");
}

#[test]
fn test_copies_exact_content() {
    let tmp_path = tempfile::tempdir().unwrap();
    let content = "Hello, World! This is a test file.";
    let data = copy_to(content, &tmp_path.path().join("copy.txt"));
    assert_eq!(data, content.as_bytes());
}

#[test]
fn test_copies_binary_file() {
    let tmp_path = tempfile::tempdir().unwrap();
    let binary_data = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
    let data = copy_to(binary_data, &tmp_path.path().join("binary.bin"));
    assert_eq!(data, binary_data);
}

#[test]
fn test_copies_empty_file() {
    let tmp_path = tempfile::tempdir().unwrap();
    let data = copy_to("", &tmp_path.path().join("empty.txt"));
    assert_eq!(data, b"");
}

#[test]
fn test_copies_large_file() {
    let tmp_path = tempfile::tempdir().unwrap();
    let content = "x".repeat(100_000);
    let data = copy_to(&content, &tmp_path.path().join("large.txt"));
    assert_eq!(data, content.as_bytes());
}

#[test]
fn test_copies_special_characters() {
    let tmp_path = tempfile::tempdir().unwrap();
    let content = "Special characters: é, ñ, ü, ö, à, 中文, 日本語";
    let data = copy_to(content, &tmp_path.path().join("special.txt"));
    assert_eq!(data, content.as_bytes());
}

#[test]
fn test_copies_to_nested_directory() {
    let tmp_path = tempfile::tempdir().unwrap();
    let target = tmp_path.path().join("nested").join("dir").join("file.txt");
    let data = copy_to("nested content", &target);
    assert_eq!(data, b"nested content");
}

#[test]
fn test_copies_with_newlines() {
    let tmp_path = tempfile::tempdir().unwrap();
    let content = "Line 1\nLine 2\nLine 3\n";
    let data = copy_to(content, &tmp_path.path().join("multiline.txt"));
    assert_eq!(data, content.as_bytes());
}
