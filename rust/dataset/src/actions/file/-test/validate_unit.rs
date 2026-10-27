use fairspec_metadata::{FairspecError, FileError, Integrity, IntegrityType, Report, Resource};
use serde_json::json;

use crate::actions::file::infer::infer_hash;
use crate::actions::file::temp::write_temp_file;
use crate::actions::file::validate::validate_file;

const LATIN1: [u8; 40] = [
    0x43, 0x61, 0x66, 0xE9, 0x20, 0x72, 0xE9, 0x73, 0x75, 0x6D, 0xE9, 0x20, 0x6E, 0x61, 0xEF, 0x76,
    0x65, 0x20, 0xE0, 0x20, 0x50, 0x61, 0x72, 0x69, 0x73, 0x2E, 0x20, 0xC7, 0x61, 0x20, 0x63, 0x27,
    0x65, 0x73, 0x74, 0x20, 0x62, 0x6F, 0x6E, 0x21,
];

fn write(content: impl AsRef<[u8]>) -> String {
    write_temp_file(content, false, None, None).unwrap()
}

fn create_resource(path: &str, textual: Option<bool>, integrity: Option<Integrity>) -> Resource {
    Resource {
        data: Some(json!(path)),
        textual,
        integrity,
        ..Resource::default()
    }
}

fn get_hash(path: &str, hash_type: IntegrityType) -> String {
    infer_hash(&create_resource(path, None, None), Some(hash_type), None).unwrap()
}

fn create_integrity(hash_type: IntegrityType, hash: &str) -> Option<Integrity> {
    Some(Integrity {
        extra: Default::default(),
        r#type: hash_type,
        hash: hash.to_string(),
    })
}

fn get_error_types(report: &Report) -> Vec<String> {
    report
        .errors
        .iter()
        .map(|error| {
            serde_json::to_value(error).unwrap()["type"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

fn assert_valid(report: &Report) {
    assert!(report.valid);
    assert!(report.errors.is_empty());
}

#[test]
fn test_validates_textual_for_utf8() {
    let path = write("Hello, World!");
    assert_valid(&validate_file(&create_resource(&path, Some(true), None), None).unwrap());
}

#[test]
fn test_validates_textual_for_ascii() {
    let path = write(b"Simple ASCII text only");
    assert_valid(&validate_file(&create_resource(&path, Some(true), None), None).unwrap());
}

#[test]
fn test_returns_error_when_textual_expected_but_binary() {
    let path = write([0xFF, 0xD8, 0xFF, 0xE0, 0x00]);
    let report = validate_file(&create_resource(&path, Some(true), None), None).unwrap();
    assert!(!report.valid);
    assert_eq!(get_error_types(&report), ["file/textual"]);
}

#[test]
fn test_returns_error_when_textual_expected_but_latin1() {
    let path = write(LATIN1);
    let report = validate_file(&create_resource(&path, Some(true), None), None).unwrap();
    assert!(!report.valid);
    assert_eq!(get_error_types(&report), ["file/textual"]);
}

#[test]
fn test_validates_integrity_md5() {
    let path = write("Hello, World!");
    let actual_hash = get_hash(&path, IntegrityType::Md5);
    let integrity = create_integrity(IntegrityType::Md5, &actual_hash);
    assert_valid(&validate_file(&create_resource(&path, None, integrity), None).unwrap());
}

#[test]
fn test_returns_error_when_integrity_mismatch() {
    let path = write("Hello, World!");
    let actual_hash = get_hash(&path, IntegrityType::Md5);
    let integrity = create_integrity(IntegrityType::Md5, "00000000000000000000000000000000");
    let report = validate_file(&create_resource(&path, None, integrity), None).unwrap();
    assert!(!report.valid);
    assert_eq!(get_error_types(&report), ["file/integrity"]);
    let FairspecError::File(FileError::Integrity(error)) = &report.errors[0] else {
        panic!("expected an integrity error");
    };
    assert_eq!(error.hash_type, "md5");
    assert_eq!(error.expected_hash, "00000000000000000000000000000000");
    assert_eq!(error.actual_hash, actual_hash);
}

#[test]
fn test_validates_sha256_integrity() {
    let path = write("Hello, World!");
    let actual_hash = get_hash(&path, IntegrityType::Sha256);
    let integrity = create_integrity(IntegrityType::Sha256, &actual_hash);
    assert_valid(&validate_file(&create_resource(&path, None, integrity), None).unwrap());
}

#[test]
fn test_validates_sha1_integrity() {
    let path = write("Hello, World!");
    let actual_hash = get_hash(&path, IntegrityType::Sha1);
    let integrity = create_integrity(IntegrityType::Sha1, &actual_hash);
    assert_valid(&validate_file(&create_resource(&path, None, integrity), None).unwrap());
}

#[test]
fn test_validates_sha512_integrity() {
    let path = write("Hello, World!");
    let actual_hash = get_hash(&path, IntegrityType::Sha512);
    let integrity = create_integrity(IntegrityType::Sha512, &actual_hash);
    assert_valid(&validate_file(&create_resource(&path, None, integrity), None).unwrap());
}

#[test]
fn test_validates_both_textual_and_integrity() {
    let path = write("Hello, World!");
    let actual_hash = get_hash(&path, IntegrityType::Md5);
    let integrity = create_integrity(IntegrityType::Md5, &actual_hash);
    let resource = create_resource(&path, Some(true), integrity);
    assert_valid(&validate_file(&resource, None).unwrap());
}

#[test]
fn test_returns_multiple_errors() {
    let path = write([0xFF, 0xD8, 0xFF, 0xE0, 0x00]);
    let integrity = create_integrity(IntegrityType::Md5, "00000000000000000000000000000000");
    let report = validate_file(&create_resource(&path, Some(true), integrity), None).unwrap();
    assert!(!report.valid);
    assert_eq!(get_error_types(&report), ["file/textual", "file/integrity"]);
}

#[test]
fn test_returns_error_only_textual_mismatch() {
    let path = write([0xFF, 0xD8, 0xFF, 0xE0, 0x00]);
    let actual_hash = get_hash(&path, IntegrityType::Md5);
    let integrity = create_integrity(IntegrityType::Md5, &actual_hash);
    let report = validate_file(&create_resource(&path, Some(true), integrity), None).unwrap();
    assert!(!report.valid);
    assert_eq!(get_error_types(&report), ["file/textual"]);
}

#[test]
fn test_returns_error_only_integrity_mismatch() {
    let path = write("Hello, World!");
    let integrity = create_integrity(IntegrityType::Md5, "00000000000000000000000000000000");
    let report = validate_file(&create_resource(&path, Some(true), integrity), None).unwrap();
    assert!(!report.valid);
    assert_eq!(get_error_types(&report), ["file/integrity"]);
}

#[test]
fn test_handles_empty_file() {
    let path = write("");
    let actual_hash = get_hash(&path, IntegrityType::Sha256);
    let integrity = create_integrity(IntegrityType::Sha256, &actual_hash);
    assert_valid(&validate_file(&create_resource(&path, None, integrity), None).unwrap());
}

#[test]
fn test_validates_textual_with_special_characters() {
    let path = write("Special: é, ñ, ü, ö, à");
    assert_valid(&validate_file(&create_resource(&path, Some(true), None), None).unwrap());
}

#[test]
fn test_reports_uppercase_hash_as_metadata_without_hashing() {
    let path = write("");
    let integrity = create_integrity(IntegrityType::Md5, "D41D8CD98F00B204E9800998ECF8427E");
    let report = validate_file(&create_resource(&path, None, integrity), None).unwrap();
    assert_eq!(get_error_types(&report), ["metadata"]);
    let FairspecError::Metadata(error) = &report.errors[0] else {
        panic!("expected a metadata error");
    };
    assert_eq!(error.json_pointer, "/integrity/hash");
}

#[test]
fn test_reports_hash_of_another_length_as_metadata() {
    let path = write("");
    let integrity = create_integrity(IntegrityType::Sha256, "d41d8cd98f00b204e9800998ecf8427e");
    let report = validate_file(&create_resource(&path, None, integrity), None).unwrap();
    assert_eq!(get_error_types(&report), ["metadata"]);
}
