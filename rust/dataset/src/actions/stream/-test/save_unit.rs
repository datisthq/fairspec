use std::fs;
use std::io::{Cursor, ErrorKind};

use fairspec_metadata::Error;

use crate::actions::stream::save::save_file_stream;

fn save(content: &[u8], path: &std::path::Path, overwrite: bool) -> fairspec_metadata::Result<()> {
    save_file_stream(Cursor::new(content), path.to_str().unwrap(), overwrite)
}

fn assert_exists_error(result: fairspec_metadata::Result<()>) {
    let Err(Error::Io(error)) = result else {
        panic!("expected an io error");
    };
    assert_eq!(error.kind(), ErrorKind::AlreadyExists);
}

#[test]
fn test_saves_stream_to_file() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("test.txt");
    save(b"Hello, World!", &path, false).unwrap();
    assert!(path.exists());
    assert_eq!(fs::read(&path).unwrap(), b"Hello, World!");
}

#[test]
fn test_saves_stream_with_multiple_chunks() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("chunks.txt");
    save(b"Hello, World!", &path, false).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"Hello, World!");
}

#[test]
fn test_creates_nested_directories() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("nested").join("dir").join("file.txt");
    save(b"Nested content", &path, false).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"Nested content");
}

#[test]
fn test_raises_when_file_exists_and_overwrite_false() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("existing.txt");
    save(b"Initial content", &path, false).unwrap();
    assert_exists_error(save(b"New content", &path, false));
}

// overwrite is a plain bool in Rust, so "not specified" is the default false
#[test]
fn test_raises_when_file_exists_and_overwrite_not_specified() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("existing2.txt");
    save(b"Initial content", &path, false).unwrap();
    assert_exists_error(save(b"New content", &path, false));
}

#[test]
fn test_overwrites_when_overwrite_true() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("overwrite.txt");
    save(b"Initial content", &path, false).unwrap();
    save(b"New content", &path, true).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"New content");
}

#[test]
fn test_saves_binary_data() {
    let tmp_path = tempfile::tempdir().unwrap();
    let binary_data = [0x00, 0x01, 0x02, 0x03, 0xFF];
    let path = tmp_path.path().join("binary.bin");
    save(&binary_data, &path, false).unwrap();
    assert_eq!(fs::read(&path).unwrap(), binary_data);
}

#[test]
fn test_saves_empty_stream() {
    let tmp_path = tempfile::tempdir().unwrap();
    let path = tmp_path.path().join("empty.txt");
    save(b"", &path, false).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"");
}

#[test]
fn test_saves_large_stream() {
    let tmp_path = tempfile::tempdir().unwrap();
    let content = vec![b'A'; 10_000];
    let path = tmp_path.path().join("large.txt");
    save(&content, &path, false).unwrap();
    assert_eq!(fs::read(&path).unwrap(), content);
}

#[test]
fn test_saves_unicode_characters() {
    let tmp_path = tempfile::tempdir().unwrap();
    let content = "Unicode: 你好世界 🌍 مرحبا".as_bytes();
    let path = tmp_path.path().join("unicode.txt");
    save(content, &path, false).unwrap();
    assert_eq!(fs::read(&path).unwrap(), content);
}
