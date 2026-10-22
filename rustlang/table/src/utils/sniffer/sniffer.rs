use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::error::SnifferError;
use super::metadata::{Dialect, Header, LineTerminator, Metadata, Quote};
use super::potential_dialects::{
    PotentialDialect, QUOTE_CHARS, detect_line_terminator, generate_potential_dialects,
};
use super::sample::{SampleSize, SampleSizeType};
use super::score::{FindBestDialectPreferences, find_best_dialect, score_dialect};
use super::table::Table;
use crate::helpers::general::format_python_float;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sniffer {
    sample_size: SampleSize,
    forced_delimiter: Option<u32>,
    forced_quote: Option<Quote>,
}

impl Default for Sniffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Sniffer {
    pub fn new() -> Self {
        Self {
            sample_size: SampleSize {
                r#type: SampleSizeType::Bytes,
                count: 8192,
            },
            forced_delimiter: None,
            forced_quote: None,
        }
    }

    pub fn with_sample_size(mut self, size: SampleSize) -> Self {
        self.sample_size = size;
        self
    }

    pub fn with_delimiter(mut self, delimiter: u32) -> Self {
        self.forced_delimiter = Some(delimiter);
        self
    }

    pub fn with_quote(mut self, quote: Quote) -> Self {
        self.forced_quote = Some(quote);
        self
    }

    pub fn sniff_bytes(&self, data: &[u8]) -> Result<Metadata, SnifferError> {
        let bytes_no_bom = skip_bom(data);
        let (current, comment_preamble_lines) = skip_preamble(bytes_no_bom);

        let sample = self.take_sample(current);
        let line_terminator = detect_line_terminator(sample);

        let dialects = match self.forced_delimiter {
            Some(delimiter) => self.generate_forced_dialects(delimiter, line_terminator),
            None => generate_potential_dialects(line_terminator),
        };

        let scores: Vec<_> = dialects
            .iter()
            .map(|dialect| score_dialect(sample, dialect))
            .collect();

        let best_score = find_best_dialect(
            &scores,
            &FindBestDialectPreferences {
                prefer_common_delimiters: true,
                prefer_double_quote: true,
            },
        )?;

        let mut dialect = Dialect {
            delimiter: best_score.dialect.delimiter,
            quote: best_score.dialect.quote,
            flexible: !best_score.is_uniform,
            is_utf8: true,
            line_terminator: best_score.dialect.line_terminator,
            header: Header {
                has_header_row: false,
                num_preamble_rows: comment_preamble_lines,
            },
        };

        let structural_preamble_rows = detect_structural_preamble(current, &dialect);
        dialect.header.num_preamble_rows += structural_preamble_rows;

        let data_after_all_preamble = skip_lines(current, structural_preamble_rows);
        dialect.header.has_header_row = detect_header(data_after_all_preamble, &dialect);

        Ok(build_metadata(bytes_no_bom, dialect))
    }

    /// Sniffs rows of JSON values written as comma-separated text, as Python's twin does.
    pub fn sniff_rows(&self, rows: &[Vec<Value>]) -> Result<Metadata, SnifferError> {
        self.sniff_bytes(convert_rows_to_csv(rows).as_bytes())
    }

    fn take_sample<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        match self.sample_size.r#type {
            SampleSizeType::All => data,
            SampleSizeType::Bytes => &data[..data.len().min(self.sample_size.count)],
            SampleSizeType::Records => {
                let mut line_count = 0;
                let mut i = 0;
                while i < data.len() && line_count < self.sample_size.count {
                    if data[i] == 10 {
                        line_count += 1;
                    } else if data[i] == 13 {
                        if data.get(i + 1) == Some(&10) {
                            i += 1;
                        }
                        line_count += 1;
                    }
                    i += 1;
                }
                &data[..i]
            }
        }
    }

    fn generate_forced_dialects(
        &self,
        delimiter: u32,
        line_terminator: LineTerminator,
    ) -> Vec<PotentialDialect> {
        let quotes = match self.forced_quote {
            Some(quote) => vec![quote],
            None => QUOTE_CHARS.to_vec(),
        };
        quotes
            .into_iter()
            .map(|quote| PotentialDialect {
                delimiter,
                quote,
                line_terminator,
            })
            .collect()
    }
}

fn skip_bom(data: &[u8]) -> &[u8] {
    data.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(data)
}

fn skip_preamble(data: &[u8]) -> (&[u8], usize) {
    let mut line_count = 0;
    let mut i = 0;

    while i < data.len() && data[i] == 35 {
        while i < data.len() && data[i] != 10 && data[i] != 13 {
            i += 1;
        }
        if data.get(i) == Some(&13) && data.get(i + 1) == Some(&10) {
            i += 2;
        } else if i < data.len() {
            i += 1;
        }
        line_count += 1;
    }

    (&data[i..], line_count)
}

fn skip_lines(data: &[u8], num_lines: usize) -> &[u8] {
    let mut line_count = 0;
    let mut i = 0;

    while i < data.len() && line_count < num_lines {
        if data[i] == 10 {
            line_count += 1;
            i += 1;
        } else if data[i] == 13 {
            i += if data.get(i + 1) == Some(&10) { 2 } else { 1 };
            line_count += 1;
        } else {
            i += 1;
        }
    }

    &data[i..]
}

fn create_potential_dialect(dialect: &Dialect) -> PotentialDialect {
    PotentialDialect {
        delimiter: dialect.delimiter,
        quote: dialect.quote,
        line_terminator: dialect.line_terminator,
    }
}

fn detect_structural_preamble(data: &[u8], dialect: &Dialect) -> usize {
    let table = Table::parse(data, &create_potential_dialect(dialect));

    if table.num_rows() < 2 {
        return 0;
    }

    let modal_field_count = table.get_modal_field_count();
    let preamble_rows = table
        .field_counts
        .iter()
        .take_while(|&&count| count != modal_field_count)
        .count();

    if preamble_rows >= table.num_rows() {
        return 0;
    }
    preamble_rows
}

fn detect_header(data: &[u8], dialect: &Dialect) -> bool {
    static BOOLEAN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)^(true|false)$").unwrap());
    static IDENTIFIER: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap());
    static UPPERCASE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Z]").unwrap());
    static INTEGER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+$").unwrap());
    // Python's `\s` also matches the information separators U+001C..U+001F
    static SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[_\s\x1C-\x1F-]").unwrap());
    static NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+(\.\d+)?$").unwrap());

    let table = Table::parse(data, &create_potential_dialect(dialect));

    if table.num_rows() < 2 {
        return false;
    }

    let first_row = &table.rows[0];
    if first_row.is_empty() {
        return false;
    }

    let mut header_score = 0.0;

    for field in first_row {
        let trimmed = strip_python_whitespace(field);

        if trimmed.is_empty() {
            header_score -= 0.2;
            continue;
        }

        if BOOLEAN.is_match(trimmed) {
            header_score -= 0.3;
            continue;
        }

        if IDENTIFIER.is_match(trimmed) {
            header_score += 0.3;
        }

        if UPPERCASE.is_match(trimmed) && !INTEGER.is_match(trimmed) {
            header_score += 0.2;
        }

        if SEPARATOR.is_match(trimmed) {
            header_score += 0.1;
        }

        if NUMBER.is_match(trimmed) {
            header_score -= 0.3;
        }
    }

    header_score > 0.0
}

fn build_metadata(data: &[u8], mut dialect: Dialect) -> Metadata {
    let data_after_preamble = skip_lines(data, dialect.header.num_preamble_rows);
    let table = Table::parse(data_after_preamble, &create_potential_dialect(&dialect));

    let data_field_counts = if dialect.header.has_header_row && !table.field_counts.is_empty() {
        &table.field_counts[1..]
    } else {
        &table.field_counts[..]
    };

    let is_uniform = match data_field_counts.first() {
        None => true,
        Some(first) => data_field_counts.iter().all(|count| count == first),
    };
    dialect.flexible = !is_uniform;

    let num_fields = table.get_modal_field_count();

    let fields = if dialect.header.has_header_row && table.num_rows() > 0 {
        table.rows[0]
            .iter()
            .map(|field| strip_python_whitespace(field).to_string())
            .collect()
    } else {
        (1..=num_fields).map(|i| format!("field_{i}")).collect()
    };

    let total_rows = table.num_rows();
    let avg_record_len = if total_rows > 0 {
        data_after_preamble.len() as f64 / total_rows as f64
    } else {
        0.0
    };

    Metadata {
        dialect,
        avg_record_len,
        num_fields,
        fields,
    }
}

/// Python's `str.strip()`: Unicode whitespace plus the separators U+001C..U+001F.
fn strip_python_whitespace(text: &str) -> &str {
    text.trim_matches(|char: char| char.is_whitespace() || ('\x1C'..='\x1F').contains(&char))
}

fn convert_rows_to_csv(rows: &[Vec<Value>]) -> String {
    rows.iter()
        .map(|row| {
            row.iter()
                .map(|value| escape_field(convert_value_to_string(value)))
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn convert_value_to_string(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(value) => value.clone(),
        Value::Bool(value) => value.to_string(),
        Value::Number(_) => format_python_number(value),
        Value::Array(_) | Value::Object(_) => dump_python_json(value),
    }
}

fn escape_field(value: String) -> String {
    if !value.contains([',', '"', '\n', '\r']) {
        return value;
    }
    format!("\"{}\"", value.replace('"', "\"\""))
}

fn format_python_number(value: &Value) -> String {
    match value {
        Value::Number(number) if number.is_f64() => {
            format_python_float(number.as_f64().unwrap_or_default())
        }
        Value::Number(number) => number.to_string(),
        _ => String::new(),
    }
}

/// Python's `json.dumps` with its default separators and ASCII escaping.
fn dump_python_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(_) => format_python_number(value),
        Value::String(value) => dump_python_string(value),
        Value::Array(items) => {
            let items: Vec<_> = items.iter().map(dump_python_json).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(entries) => {
            let entries: Vec<_> = entries
                .iter()
                .map(|(key, value)| {
                    format!("{}: {}", dump_python_string(key), dump_python_json(value))
                })
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
    }
}

fn dump_python_string(text: &str) -> String {
    let mut output = String::from("\"");
    for char in text.chars() {
        match char {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{8}' => output.push_str("\\b"),
            '\u{c}' => output.push_str("\\f"),
            ' '..='~' => output.push(char),
            _ => {
                let mut units = [0; 2];
                for unit in char.encode_utf16(&mut units) {
                    output.push_str(&format!("\\u{unit:04x}"));
                }
            }
        }
    }
    output.push('"');
    output
}
