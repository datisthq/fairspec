#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineTerminator {
    Lf,
    Crlf,
    Cr,
}

impl LineTerminator {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineTerminator::Lf => "\n",
            LineTerminator::Crlf => "\r\n",
            LineTerminator::Cr => "\r",
        }
    }
}

/// A quote character as a code point; `None` means fields are never quoted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Quote {
    pub char: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub has_header_row: bool,
    pub num_preamble_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dialect {
    pub delimiter: u32,
    pub header: Header,
    pub quote: Quote,
    pub flexible: bool,
    pub is_utf8: bool,
    pub line_terminator: LineTerminator,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub dialect: Dialect,
    pub avg_record_len: f64,
    pub num_fields: usize,
    pub fields: Vec<String>,
}
