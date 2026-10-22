use super::metadata::{LineTerminator, Quote};

/// Comma, semicolon, tab, pipe, caret, tilde, hash, ampersand, section sign and slash.
pub const DELIMITERS: [u32; 10] = [44, 59, 9, 124, 94, 126, 35, 38, 167, 47];

pub const QUOTE_CHARS: [Quote; 3] = [
    Quote { char: None },
    Quote { char: Some(34) },
    Quote { char: Some(39) },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PotentialDialect {
    pub delimiter: u32,
    pub quote: Quote,
    pub line_terminator: LineTerminator,
}

pub fn detect_line_terminator(data: &[u8]) -> LineTerminator {
    let mut has_cr = false;
    let mut has_lf = false;
    let mut has_crlf = false;

    let mut i = 0;
    while i < data.len() {
        if data[i] == 13 {
            if data.get(i + 1) == Some(&10) {
                has_crlf = true;
                i += 1;
            } else {
                has_cr = true;
            }
        } else if data[i] == 10 {
            has_lf = true;
        }
        i += 1;
    }

    if has_crlf {
        LineTerminator::Crlf
    } else if has_lf {
        LineTerminator::Lf
    } else if has_cr {
        LineTerminator::Cr
    } else {
        LineTerminator::Lf
    }
}

pub fn generate_potential_dialects(line_terminator: LineTerminator) -> Vec<PotentialDialect> {
    DELIMITERS
        .iter()
        .flat_map(|&delimiter| {
            QUOTE_CHARS.iter().map(move |&quote| PotentialDialect {
                delimiter,
                quote,
                line_terminator,
            })
        })
        .collect()
}
