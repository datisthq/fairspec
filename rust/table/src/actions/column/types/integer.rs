use fairspec_metadata::{BaseIntegerColumnProperty, IntegerColumn};
use polars::prelude::*;

use super::number::NUMBER_PATTERN;
use crate::actions::column::_shared::text::strip_number_text;

const PLAIN_INTEGER_PATTERN: &str = r"^-?(?:0|[1-9][0-9]*)$";
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
    let plain_expr = when(
        text_expr
            .clone()
            .str()
            .contains(lit(PLAIN_INTEGER_PATTERN), false),
    )
    .then(text_expr.clone())
    .otherwise(lit(NULL))
    .cast(DataType::Int64);

    // Only cells the plain form missed reach the exact reading, so the rest stay null there
    let missed_expr = when(plain_expr.clone().is_null())
        .then(text_expr)
        .otherwise(lit(NULL));
    let other_expr = when(
        missed_expr
            .clone()
            .str()
            .contains(lit(NUMBER_PATTERN), false),
    )
    .then(missed_expr)
    .otherwise(lit(NULL));

    let fraction_expr = other_expr
        .clone()
        .str()
        .extract(lit(r"\.([0-9]+)"), 1)
        .fill_null(lit(""));
    let digits_expr = concat_str(
        [
            other_expr.clone().str().extract(lit(r"^-?([0-9]+)"), 1),
            fraction_expr.clone(),
        ],
        "",
        false,
    )
    .str()
    .strip_chars_start(lit("0"));
    let trimmed_expr = digits_expr.clone().str().strip_chars_end(lit("0"));
    let exponent_expr = other_expr
        .clone()
        .str()
        .extract(lit(r"[eE]([+-]?[0-9]+)$"), 1)
        .fill_null(lit("0"))
        .cast(DataType::Int64)
        .clip(lit(-MAX_EXPONENT), lit(MAX_EXPONENT));
    let shift_expr = exponent_expr - get_length(fraction_expr) + get_length(digits_expr)
        - get_length(trimmed_expr.clone());
    let length_expr = get_length(trimmed_expr.clone()) + shift_expr.clone();
    let is_whole_expr = shift_expr
        .clone()
        .gt_eq(lit(0))
        .and(length_expr.clone().lt_eq(lit(MAX_INTEGER_DIGITS)));

    let padded_expr = concat_str(
        [
            trimmed_expr.clone(),
            lit("0".repeat(MAX_INTEGER_DIGITS as usize)),
        ],
        "",
        false,
    )
    .str()
    .slice(
        lit(0),
        when(is_whole_expr.clone())
            .then(length_expr)
            .otherwise(lit(0i64))
            .cast(DataType::UInt64),
    );
    let whole_expr = when(trimmed_expr.clone().eq(lit("")))
        .then(lit("0"))
        .when(is_whole_expr)
        .then(padded_expr)
        .otherwise(lit(NULL));
    let sign_expr = when(other_expr.str().starts_with(lit("-")))
        .then(lit("-"))
        .otherwise(lit(""));
    let other_value_expr = concat_str([sign_expr, whole_expr], "", false).cast(DataType::Int64);

    plain_expr.fill_null(other_value_expr)
}

fn get_length(text_expr: Expr) -> Expr {
    text_expr.str().len_chars().cast(DataType::Int64)
}
