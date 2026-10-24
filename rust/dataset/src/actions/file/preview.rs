use fairspec_metadata::{Error, Result};

use super::infer::get_is_binary;
use super::load::load_file;
use crate::models::file::FilePreview;

/// Reads the raw source of a local or remote file as UTF-8 text, up to the first
/// `max_lines` lines (default 20) or `max_bytes` bytes (default 10,000), whichever comes first.
pub fn preview_file(
    path: &str,
    max_bytes: Option<usize>,
    max_lines: Option<usize>,
) -> Result<FilePreview> {
    let max_bytes = max_bytes.unwrap_or(10_000);
    let max_lines = max_lines.unwrap_or(20);
    assert_positive_integer("max_bytes", max_bytes)?;
    assert_positive_integer("max_lines", max_lines)?;

    // one extra byte tells a file of exactly max_bytes apart from a longer one
    let buffer = load_file(path, Some(max_bytes + 1))?;
    let sample = &buffer[..buffer.len().min(max_bytes)];
    let mut truncated = buffer.len() > max_bytes;

    if get_is_binary(sample) {
        return Err(Error::Load("Cannot preview a binary file".to_string()));
    }

    let mut text = decode_utf8(sample, !truncated);

    let mut end = None;
    for _ in 0..max_lines {
        let start = end.map_or(0, |end| end + 1);
        match text[start..].find('\n') {
            Some(offset) => end = Some(start + offset),
            None => {
                end = None;
                break;
            }
        }
    }
    if let Some(end) = end.filter(|end| end + 1 < text.len()) {
        text.truncate(end + 1);
        truncated = true;
    }

    Ok(FilePreview { text, truncated })
}

// Python's incremental "replace" decoder: a non-final decode keeps back a
// multi-byte character cut by the byte limit instead of replacing it
fn decode_utf8(data: &[u8], is_final: bool) -> String {
    let mut text = String::with_capacity(data.len());
    let mut chunks = data.utf8_chunks().peekable();
    while let Some(chunk) = chunks.next() {
        text.push_str(chunk.valid());
        let invalid = chunk.invalid();
        if invalid.is_empty() {
            continue;
        }
        let is_cut = chunks.peek().is_none()
            && std::str::from_utf8(invalid).is_err_and(|error| error.error_len().is_none());
        if !(is_cut && !is_final) {
            text.push(char::REPLACEMENT_CHARACTER);
        }
    }
    text
}

fn assert_positive_integer(name: &str, value: usize) -> Result<()> {
    if value == 0 {
        return Err(Error::Load(format!("{name} must be a positive integer")));
    }
    Ok(())
}
