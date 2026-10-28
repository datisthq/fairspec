use fairspec_metadata::{BaseIntegerColumnProperty, IntegerColumn};
use polars::prelude::*;

use crate::actions::column::_shared::text::strip_number_text;

const MAX_INTEGER_DIGITS: i64 = 19;
// Keeps the exponent arithmetic below from wrapping Int64 into a huge zero padding
const MAX_EXPONENT: i64 = 1_000_000_000_000_000;

pub fn parse_integer_column(column: &IntegerColumn, column_expr: Expr) -> Expr {
    parse_integer_text(normalize_integer_text(&column.property.base, column_expr))
}

pub fn stringify_integer_column(_column: &IntegerColumn, column_expr: Expr) -> Expr {
    column_expr.cast(DataType::String)
}

pub fn normalize_integer_text(property: &BaseIntegerColumnProperty, column_expr: Expr) -> Expr {
    let mut column_expr = column_expr;
    if property.with_text == Some(true) {
        column_expr = strip_number_text(column_expr, None);
    }
    if let Some(group_char) = property.group_char.as_deref().filter(|c| !c.is_empty()) {
        column_expr = column_expr
            .str()
            .replace_all(lit(regex::escape(group_char)), lit(""), false);
    }
    column_expr
}

/// Read a JSON number whose fraction is zero as an Int64, or null outside its range
pub fn parse_integer_text(text_expr: Expr) -> Expr {
    // One native pass: the regex, extract and padding steps it replaces each cost a node
    // per morsel, and the streaming engine evaluates a reused step once per use
    text_expr.map(
        |batch: Column| {
            let text = batch.as_materialized_series().cast(&DataType::String)?;
            let values: Int64Chunked = text
                .str()?
                .iter()
                .map(|cell| cell.and_then(parse_integer))
                .collect();
            Ok(values.with_name(batch.name().clone()).into_column())
        },
        |_, field| Ok(Field::new(field.name().clone(), DataType::Int64)),
    )
}

/// A JSON number (`NUMBER_PATTERN`) whose value is a whole Int64, read exactly
pub(crate) fn parse_integer(text: &str) -> Option<i64> {
    let (is_negative, text) = match text.strip_prefix('-') {
        Some(rest) => (true, rest.as_bytes()),
        None => (false, text.as_bytes()),
    };
    let integer = take_digits(text);
    if integer.is_empty() || (integer.len() > 1 && integer[0] == b'0') {
        return None;
    }
    let mut rest = &text[integer.len()..];

    let mut fraction: &[u8] = &[];
    if let Some(after) = rest.strip_prefix(b".") {
        fraction = take_digits(after);
        if fraction.is_empty() {
            return None;
        }
        rest = &after[fraction.len()..];
    }

    let mut exponent = Some(0);
    if let Some(after) = rest.strip_prefix(b"e").or_else(|| rest.strip_prefix(b"E")) {
        let (is_exponent_negative, after) = match after.first() {
            Some(b'-') => (true, &after[1..]),
            Some(b'+') => (false, &after[1..]),
            _ => (false, after),
        };
        let digits = take_digits(after);
        if digits.is_empty() {
            return None;
        }
        rest = &after[digits.len()..];
        exponent = read_exponent(digits, is_exponent_negative);
    }
    if !rest.is_empty() {
        return None;
    }

    // The value is its significant digits (`core`) times 10 to the `shift`, whole when the
    // shift is not negative; i128 holds every product of at most 19 digits
    let significant = trim_zeros(integer, fraction, true);
    let core_length = trim_zeros(integer, fraction, false);
    if core_length == 0 {
        return Some(0);
    }
    let exponent = exponent?;
    let shift = exponent - fraction.len() as i64 + (significant - core_length) as i64;
    if shift < 0 || core_length as i64 + shift > MAX_INTEGER_DIGITS {
        return None;
    }
    let mut magnitude: i128 = 0;
    for &digit in integer
        .iter()
        .chain(fraction)
        .skip(integer.len() + fraction.len() - significant)
        .take(core_length)
    {
        magnitude = magnitude * 10 + i128::from(digit - b'0');
    }
    let magnitude = magnitude * 10i128.pow(shift as u32);
    i64::try_from(if is_negative { -magnitude } else { magnitude }).ok()
}

fn take_digits(text: &[u8]) -> &[u8] {
    let end = text
        .iter()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(text.len());
    &text[..end]
}

// The length of the integer and fraction digits with leading zeros dropped, and with
// trailing ones too unless `is_significant`
fn trim_zeros(integer: &[u8], fraction: &[u8], is_significant: bool) -> usize {
    let digits = || integer.iter().chain(fraction);
    let total = integer.len() + fraction.len();
    let leading = digits().take_while(|digit| **digit == b'0').count();
    if is_significant || leading == total {
        return total - leading;
    }
    let trailing = fraction
        .iter()
        .rev()
        .chain(integer.iter().rev())
        .take_while(|digit| **digit == b'0')
        .count();
    total - leading - trailing
}

// None past Int64, as the cast of the exponent text is; clipped so the shift cannot wrap
fn read_exponent(digits: &[u8], is_negative: bool) -> Option<i64> {
    let mut value: i128 = 0;
    for &digit in digits {
        value = value * 10 + i128::from(digit - b'0');
        if value > i128::from(i64::MAX) + 1 {
            return None;
        }
    }
    let value = if is_negative { -value } else { value };
    let value = i64::try_from(value).ok()?;
    Some(value.clamp(-MAX_EXPONENT, MAX_EXPONENT))
}
