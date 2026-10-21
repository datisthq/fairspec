use fairspec_metadata::{IntegrityType, Resource};
use serde_json::json;

use crate::actions::file::infer::{infer_bytes, infer_hash, infer_textual};
use crate::actions::file::temp::write_temp_file;

const LATIN1: [u8; 40] = [
    0x43, 0x61, 0x66, 0xE9, 0x20, 0x72, 0xE9, 0x73, 0x75, 0x6D, 0xE9, 0x20, 0x6E, 0x61, 0xEF, 0x76,
    0x65, 0x20, 0xE0, 0x20, 0x50, 0x61, 0x72, 0x69, 0x73, 0x2E, 0x20, 0xC7, 0x61, 0x20, 0x63, 0x27,
    0x65, 0x73, 0x74, 0x20, 0x62, 0x6F, 0x6E, 0x21,
];

const WINDOWS_1252: [u8; 17] = [
    0x43, 0x61, 0x66, 0xE9, 0x20, 0x6E, 0x61, 0xEF, 0x76, 0x65, 0x20, 0x72, 0xE9, 0x73, 0x75, 0x6D,
    0xE9,
];

fn create_resource(content: impl AsRef<[u8]>) -> Resource {
    let path = write_temp_file(content, false, None, None).unwrap();
    create_multipart_resource(&[path])
}

fn create_multipart_resource(paths: &[String]) -> Resource {
    let data = match paths {
        [path] => json!(path),
        paths => json!(paths),
    };
    Resource {
        data: Some(data),
        ..Resource::default()
    }
}

#[test]
fn test_computes_sha256_hash_by_default() {
    let result = infer_hash(&create_resource("Hello, World!"), None, None).unwrap();
    assert_eq!(result.len(), 64);
}

#[test]
fn test_computes_md5_hash() {
    let resource = create_resource("Hello, World!");
    let result = infer_hash(&resource, Some(IntegrityType::Md5), None).unwrap();
    assert_eq!(result.len(), 32);
}

#[test]
fn test_computes_sha1_hash() {
    let resource = create_resource("Hello, World!");
    let result = infer_hash(&resource, Some(IntegrityType::Sha1), None).unwrap();
    assert_eq!(result.len(), 40);
}

#[test]
fn test_computes_sha512_hash() {
    let resource = create_resource("Hello, World!");
    let result = infer_hash(&resource, Some(IntegrityType::Sha512), None).unwrap();
    assert_eq!(result.len(), 128);
}

#[test]
fn test_consistent_hashes_for_same_content() {
    let resource = create_resource("Hello, World!");
    let result1 = infer_hash(&resource, None, None).unwrap();
    let result2 = infer_hash(&resource, None, None).unwrap();
    assert_eq!(result1, result2);
}

#[test]
fn test_returns_file_size() {
    assert_eq!(
        infer_bytes(&create_resource("Hello, World!"), None).unwrap(),
        13
    );
}

#[test]
fn test_handles_empty_files() {
    assert_eq!(infer_bytes(&create_resource(""), None).unwrap(), 0);
}

#[test]
fn test_handles_larger_files() {
    let resource = create_resource("x".repeat(10_000));
    assert_eq!(infer_bytes(&resource, None).unwrap(), 10_000);
}

#[test]
fn test_handles_binary_data() {
    let resource = create_resource([0xFF, 0xD8, 0xFF, 0xE0]);
    assert_eq!(infer_bytes(&resource, None).unwrap(), 4);
}

#[test]
fn test_returns_true_for_utf8_text() {
    let resource = create_resource("Hello, World! This is UTF-8 text.");
    assert!(infer_textual(&resource, None).unwrap());
}

#[test]
fn test_returns_false_for_binary() {
    let resource = create_resource([0xFF, 0xD8, 0xFF, 0xE0, 0x00]);
    assert!(!infer_textual(&resource, None).unwrap());
}

#[test]
fn test_uses_custom_sample_bytes() {
    let resource = create_resource("This is a test file with UTF-8 content.");
    assert!(infer_textual(&resource, Some(20)).unwrap());
}

#[test]
fn test_handles_large_text_files() {
    let resource = create_resource("Hello World! ".repeat(1000));
    assert!(infer_textual(&resource, None).unwrap());
}

#[test]
fn test_infer_textual_handles_empty_files() {
    assert!(infer_textual(&create_resource(""), None).unwrap());
}

#[test]
fn test_handles_special_characters() {
    let resource = create_resource("Special: é, ñ, ü, ö, à");
    assert!(infer_textual(&resource, None).unwrap());
}

#[test]
fn test_returns_true_for_ascii() {
    let resource = create_resource(b"Simple ASCII text only");
    assert!(infer_textual(&resource, None).unwrap());
}

#[test]
fn test_returns_true_for_utf8_with_unicode() {
    let resource = create_resource("Héllo, Wörld! 你好 مرحبا 🌍");
    assert!(infer_textual(&resource, None).unwrap());
}

#[test]
fn test_returns_true_for_cyrillic() {
    assert!(infer_textual(&create_resource("Привет мир"), None).unwrap());
}

#[test]
fn test_returns_true_for_japanese() {
    assert!(infer_textual(&create_resource("こんにちは世界"), None).unwrap());
}

#[test]
fn test_returns_true_for_arabic() {
    assert!(infer_textual(&create_resource("مرحبا بالعالم"), None).unwrap());
}

#[test]
fn test_returns_false_for_latin1() {
    assert!(!infer_textual(&create_resource(LATIN1), None).unwrap());
}

#[test]
fn test_returns_false_for_windows_1252() {
    assert!(!infer_textual(&create_resource(WINDOWS_1252), None).unwrap());
}

#[test]
fn test_requires_every_part_to_be_utf8() {
    let first = write_temp_file("Hello\n", false, None, None).unwrap();
    let latin1: Vec<u8> = "Café résumé naïve à Paris."
        .chars()
        .map(|char| u8::try_from(u32::from(char)).unwrap())
        .collect();
    let second = write_temp_file(latin1, false, None, None).unwrap();
    let both_utf8 = create_multipart_resource(&[first.clone(), first.clone()]);
    let mixed = create_multipart_resource(&[first, second]);
    assert!(infer_textual(&both_utf8, None).unwrap());
    assert!(!infer_textual(&mixed, None).unwrap());
}

#[test]
fn test_hashes_multipart_identically_under_concurrency() {
    let paths: Vec<String> = (0..4)
        .map(|index| write_temp_file(format!("part-{index}\n"), false, None, None).unwrap())
        .collect();
    let resource = create_multipart_resource(&paths);

    let serial = infer_hash(&resource, None, Some(1)).unwrap();
    let concurrent = infer_hash(&resource, None, Some(4)).unwrap();

    assert_eq!(serial, concurrent);
    assert_eq!(serial.len(), 64);
}

#[test]
fn test_hashes_multipart_in_path_order() {
    let first = write_temp_file("a", false, None, None).unwrap();
    let second = write_temp_file("b", false, None, None).unwrap();

    let forward = create_multipart_resource(&[first.clone(), second.clone()]);
    let backward = create_multipart_resource(&[second, first]);

    assert_ne!(
        infer_hash(&forward, None, Some(4)).unwrap(),
        infer_hash(&backward, None, Some(4)).unwrap()
    );
}
