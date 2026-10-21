use std::io::{Cursor, Read};

use crate::actions::stream::concat::concat_file_streams;
use crate::actions::stream::load::FileStream;

fn create_stream(data: impl Into<Vec<u8>>) -> FileStream {
    Box::new(Cursor::new(data.into()))
}

fn concat(streams: Vec<FileStream>) -> Vec<u8> {
    let mut data = Vec::new();
    concat_file_streams(streams)
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();
    data
}

#[test]
fn test_concatenates_multiple_streams_in_order() {
    let streams = vec![
        create_stream("Hello, "),
        create_stream("World"),
        create_stream("!"),
    ];
    assert_eq!(concat(streams), b"Hello, World!");
}

#[test]
fn test_handles_single_stream() {
    let streams = vec![create_stream("Single stream content")];
    assert_eq!(concat(streams), b"Single stream content");
}

#[test]
fn test_handles_empty_array() {
    assert_eq!(concat(vec![]), b"");
}

#[test]
fn test_handles_streams_with_empty_content() {
    let streams = vec![
        create_stream(""),
        create_stream("Content"),
        create_stream(""),
    ];
    assert_eq!(concat(streams), b"Content");
}

#[test]
fn test_concatenates_streams_with_multiple_chunks() {
    let streams = vec![create_stream("ABC"), create_stream("DEF")];
    assert_eq!(concat(streams), b"ABCDEF");
}

#[test]
fn test_handles_binary_data() {
    let streams = vec![
        create_stream([0x00, 0x01, 0x02]),
        create_stream([0x03, 0x04, 0x05]),
    ];
    assert_eq!(concat(streams), [0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
}

#[test]
fn test_handles_large_streams() {
    let content1 = "A".repeat(5000);
    let content2 = "B".repeat(5000);
    let streams = vec![
        create_stream(content1.clone()),
        create_stream(content2.clone()),
    ];
    let data = concat(streams);
    assert_eq!(data.len(), 10_000);
    assert_eq!(data, (content1 + &content2).into_bytes());
}

#[test]
fn test_preserves_unicode_characters() {
    let streams = vec![
        create_stream("Hello 世界"),
        create_stream(" مرحبا"),
        create_stream(" 🌍"),
    ];
    assert_eq!(
        String::from_utf8(concat(streams)).unwrap(),
        "Hello 世界 مرحبا 🌍"
    );
}

#[test]
fn test_maintains_stream_order_with_many_streams() {
    let streams = (0..10).map(|i| create_stream(i.to_string())).collect();
    assert_eq!(concat(streams), b"0123456789");
}
