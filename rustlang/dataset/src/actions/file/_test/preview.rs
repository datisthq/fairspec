use crate::actions::file::preview::preview_file;
use crate::actions::file::temp::write_temp_file;

fn create_lines(count: usize) -> Vec<String> {
    (0..count).map(|i| format!("{i},row{i}\n")).collect()
}

fn write(content: impl AsRef<[u8]>) -> String {
    write_temp_file(content, false, None, None).unwrap()
}

#[test]
fn test_returns_small_file_whole() {
    let preview = preview_file(&write("id,name\n1,alice\n"), None, None).unwrap();
    assert_eq!(preview.text, "id,name\n1,alice\n");
    assert!(!preview.truncated);
}

#[test]
fn test_keeps_first_20_lines_by_default() {
    let lines = create_lines(50);
    let preview = preview_file(&write(lines.concat()), None, None).unwrap();
    assert_eq!(preview.text, lines[..20].concat());
    assert!(preview.truncated);
}

#[test]
fn test_cuts_at_max_lines() {
    let lines = create_lines(10);
    let preview = preview_file(&write(lines.concat()), None, Some(3)).unwrap();
    assert_eq!(preview.text, lines[..3].concat());
    assert!(preview.truncated);
}

#[test]
fn test_cuts_at_max_bytes_before_max_lines() {
    let preview = preview_file(&write("a".repeat(100)), Some(10), None).unwrap();
    assert_eq!(preview.text, "a".repeat(10));
    assert!(preview.truncated);
}

#[test]
fn test_file_of_exactly_max_bytes_is_not_truncated() {
    let preview = preview_file(&write("a".repeat(10)), Some(10), None).unwrap();
    assert_eq!(preview.text, "a".repeat(10));
    assert!(!preview.truncated);
}

#[test]
fn test_file_of_exactly_max_lines_is_not_truncated() {
    let lines = create_lines(3);
    let preview = preview_file(&write(lines.concat()), None, Some(3)).unwrap();
    assert_eq!(preview.text, lines.concat());
    assert!(!preview.truncated);
}

#[test]
fn test_drops_multi_byte_character_cut_by_max_bytes() {
    let content = format!("{}é", "a".repeat(99));
    let preview = preview_file(&write(content), Some(100), None).unwrap();
    assert_eq!(preview.text, "a".repeat(99));
    assert!(preview.truncated);
}

// usize cannot hold Python's max_lines=-1, so zero stands in for both limits
#[test]
fn test_rejects_non_positive_limits() {
    let path = write("id\n");
    let error = preview_file(&path, Some(0), None).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("max_bytes must be a positive integer")
    );
    let error = preview_file(&path, None, Some(0)).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("max_lines must be a positive integer")
    );
}

#[test]
fn test_rejects_binary_file() {
    let path = write([0x00, 0x01, 0x02, 0x03, 0xFF, 0x00]);
    let error = preview_file(&path, None, None).unwrap_err();
    assert!(error.to_string().contains("Cannot preview a binary file"));
}
