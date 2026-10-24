use crate::utils::sniffer::metadata::LineTerminator;
use crate::utils::sniffer::potential_dialects::{
    DELIMITERS, QUOTE_CHARS, detect_line_terminator, generate_potential_dialects,
};

#[test]
fn test_detect_lf() {
    assert_eq!(
        detect_line_terminator(b"line1\nline2\nline3"),
        LineTerminator::Lf
    );
}

#[test]
fn test_detect_crlf() {
    assert_eq!(
        detect_line_terminator(b"line1\r\nline2\r\nline3"),
        LineTerminator::Crlf
    );
}

#[test]
fn test_detect_cr() {
    assert_eq!(
        detect_line_terminator(b"line1\rline2\rline3"),
        LineTerminator::Cr
    );
}

#[test]
fn test_prefer_crlf_over_lf() {
    assert_eq!(
        detect_line_terminator(b"line1\r\nline2\nline3"),
        LineTerminator::Crlf
    );
}

#[test]
fn test_prefer_lf_over_cr() {
    assert_eq!(
        detect_line_terminator(b"line1\nline2\rline3"),
        LineTerminator::Lf
    );
}

#[test]
fn test_default_lf_for_empty() {
    assert_eq!(detect_line_terminator(b""), LineTerminator::Lf);
}

#[test]
fn test_default_lf_for_single_line() {
    assert_eq!(
        detect_line_terminator(b"single line without terminator"),
        LineTerminator::Lf
    );
}

#[test]
fn test_generates_all_combinations() {
    let dialects = generate_potential_dialects(LineTerminator::Lf);
    assert_eq!(dialects.len(), DELIMITERS.len() * QUOTE_CHARS.len());
}

#[test]
fn test_uses_provided_line_terminator() {
    let dialects = generate_potential_dialects(LineTerminator::Crlf);
    assert!(
        dialects
            .iter()
            .all(|d| d.line_terminator == LineTerminator::Crlf)
    );
}

#[test]
fn test_includes_all_delimiters() {
    let dialects = generate_potential_dialects(LineTerminator::Lf);
    for delimiter in DELIMITERS {
        assert!(dialects.iter().any(|d| d.delimiter == delimiter));
    }
}

#[test]
fn test_includes_all_quote_chars() {
    let dialects = generate_potential_dialects(LineTerminator::Lf);
    assert!(dialects.iter().any(|d| d.quote.char.is_none()));
    assert!(dialects.iter().any(|d| d.quote.char == Some(34)));
    assert!(dialects.iter().any(|d| d.quote.char == Some(39)));
}

#[test]
fn test_generates_exactly_30() {
    assert_eq!(generate_potential_dialects(LineTerminator::Lf).len(), 30);
}

#[test]
fn test_includes_common_delimiters() {
    for delimiter in [44, 9, 59, 124] {
        assert!(DELIMITERS.contains(&delimiter));
    }
}

#[test]
fn test_has_10_delimiters() {
    assert_eq!(DELIMITERS.len(), 10);
}

#[test]
fn test_includes_none() {
    assert!(QUOTE_CHARS.iter().any(|q| q.char.is_none()));
}

#[test]
fn test_includes_double_quote() {
    assert!(QUOTE_CHARS.iter().any(|q| q.char == Some(34)));
}

#[test]
fn test_includes_single_quote() {
    assert!(QUOTE_CHARS.iter().any(|q| q.char == Some(39)));
}

#[test]
fn test_has_3_options() {
    assert_eq!(QUOTE_CHARS.len(), 3);
}
