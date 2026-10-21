use std::io::Read;

use crate::actions::file::temp::write_temp_file;
use crate::actions::stream::load::{FileStream, load_file_stream};

fn write(content: impl AsRef<[u8]>) -> String {
    write_temp_file(content, false, None, None).unwrap()
}

fn read(stream: FileStream) -> Vec<u8> {
    let mut stream = stream;
    let mut data = Vec::new();
    stream.read_to_end(&mut data).unwrap();
    data
}

#[test]
fn test_loads_stream_from_single_local_file() {
    let path = write("Hello, World!");
    let stream = load_file_stream(path, None, None).unwrap();
    assert_eq!(read(stream), b"Hello, World!");
}

#[test]
fn test_loads_stream_from_array_using_default_index() {
    let file1 = write("First file content");
    let file2 = write("Second file content");
    let stream = load_file_stream(vec![file1, file2], None, None).unwrap();
    assert_eq!(read(stream), b"First file content");
}

#[test]
fn test_loads_stream_from_array_using_specified_index() {
    let file1 = write("First file content");
    let file2 = write("Second file content");
    let stream = load_file_stream(vec![file1, file2], Some(1), None).unwrap();
    assert_eq!(read(stream), b"Second file content");
}

#[test]
fn test_limits_stream_to_max_bytes() {
    let path = write("This is a long content that should be truncated");
    let data = read(load_file_stream(path, None, Some(10)).unwrap());
    assert_eq!(data, b"This is a ");
    assert_eq!(data.len(), 10);
}

#[test]
fn test_raises_error_for_invalid_index() {
    let path = write("content");
    let error = load_file_stream(vec![path], Some(5), None).err().unwrap();
    assert!(error.to_string().contains("Cannot stream resource"));
}

#[test]
fn test_raises_error_for_empty_array() {
    let error = load_file_stream(Vec::<String>::new(), Some(0), None)
        .err()
        .unwrap();
    assert!(error.to_string().contains("Cannot stream resource"));
}

#[test]
fn test_handles_large_files() {
    let content = "A".repeat(10_000);
    let data = read(load_file_stream(write(&content), None, None).unwrap());
    assert_eq!(data.len(), 10_000);
    assert_eq!(data, content.as_bytes());
}

#[test]
fn test_handles_binary_content() {
    let binary_data = [0x00, 0x01, 0x02, 0x03, 0xFF];
    let data = read(load_file_stream(write(binary_data), None, None).unwrap());
    assert_eq!(data, binary_data);
}

#[test]
fn test_handles_empty_files() {
    let data = read(load_file_stream(write(""), None, None).unwrap());
    assert_eq!(data, b"");
}

#[test]
fn test_limits_bytes_correctly() {
    let path = write("0123456789ABCDEFGHIJ");
    assert_eq!(
        read(load_file_stream(path, None, Some(5)).unwrap()),
        b"01234"
    );
}

#[test]
fn test_handles_max_bytes_larger_than_file() {
    let path = write("Short");
    assert_eq!(
        read(load_file_stream(path, None, Some(1000)).unwrap()),
        b"Short"
    );
}
