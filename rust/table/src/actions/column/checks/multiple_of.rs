use fairspec_metadata::{CellError, CellMultipleOfError, Column};
use polars::prelude::*;
use serde_json::Value;

use super::check::ColumnCheck;
use crate::actions::column::types::decimal::normalize_number_text;
use crate::helpers::general::format_python_float;
use crate::models::cell::CellMapping;

const MAX_DECIMAL_DIGITS: usize = 38;
const INTEGER_DECIMAL: DataType = DataType::Decimal(MAX_DECIMAL_DIGITS, 0);
const MAX_VALUE_DIGITS: usize = MAX_DECIMAL_DIGITS - 1;

pub fn check_cell_multiple_of(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let Some(Value::Number(multiple_of)) = property.get("multipleOf") else {
        return None;
    };

    let error_template = CellError::MultipleOf(CellMultipleOfError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        multiple_of: multiple_of.clone(),
    });

    let divisor_number = multiple_of.as_f64().unwrap_or_default();
    let is_float_error_expr = (mapping.target.clone().cast(DataType::Float64)
        % lit(divisor_number))
    .eq(lit(0))
    .not();

    let divisor_text = match multiple_of.as_i64() {
        Some(integer) => integer.to_string(),
        None => expand_exponent(&format_python_float(divisor_number)),
    };
    let (integer_digits, fraction_digits) =
        divisor_text.split_once('.').unwrap_or((&divisor_text, ""));
    let scale = fraction_digits.len();
    let divisor = format!("{integer_digits}{fraction_digits}")
        .trim_start_matches('0')
        .to_string();
    if divisor.is_empty() || divisor.len() > MAX_DECIMAL_DIGITS || scale >= MAX_VALUE_DIGITS {
        return Some(ColumnCheck {
            is_error_expr: is_float_error_expr,
            error_template,
        });
    }

    // The value scaled by 10^scale must be an integer of at most 37 digits: the quotient rounds,
    // so multiplied back it can pass 38 and abort the query; anything else is checked as a float
    let text_expr = select_value_text(column, mapping);
    let is_exact_expr = text_expr
        .clone()
        .str()
        .contains(
            lit(format!(
                r"^[+-]?0*[0-9]{{0,{}}}(?:\.[0-9]*)?$",
                MAX_VALUE_DIGITS - scale
            )),
            false,
        )
        .and(text_expr.clone().str().contains(lit("[0-9]"), false));

    let has_excess_digits_expr = text_expr
        .clone()
        .str()
        .contains(lit(format!(r"\.[0-9]{{{scale}}}[0-9]*[1-9]")), false);
    let integer_expr = text_expr
        .clone()
        .str()
        .replace(lit(r"\..*$"), lit(""), false)
        .str()
        .replace(lit("^([+-]?)"), lit("${1}0"), false);
    let fraction_expr = concat_str(
        [
            text_expr
                .str()
                .extract(lit(r"\.([0-9]*)"), 1)
                .fill_null(lit("")),
            lit("0".repeat(scale)),
        ],
        "",
        false,
    )
    .str()
    .slice(lit(0), lit(scale as u64));

    let scaled_expr = when(is_exact_expr.clone())
        .then(concat_str([integer_expr, fraction_expr], "", false))
        .otherwise(lit("0"))
        .cast(INTEGER_DECIMAL);

    let divisor_expr = lit(divisor).cast(INTEGER_DECIMAL);
    let is_exact_error_expr = has_excess_digits_expr
        .or((scaled_expr.clone() / divisor_expr.clone() * divisor_expr).neq(scaled_expr));

    let is_error_expr = when(is_exact_expr)
        .then(is_exact_error_expr)
        .otherwise(is_float_error_expr);

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}

// A float target keeps no digit past double precision, so the text as written is used
fn select_value_text(column: &Column, mapping: &CellMapping) -> Expr {
    let target_text_expr = mapping.target.clone().cast(DataType::String);
    let (decimal_char, group_char, with_text) = match column {
        Column::Number(column) => {
            let property = &column.property.base;
            (
                &property.decimal_char,
                &property.group_char,
                property.with_text,
            )
        }
        Column::Decimal(column) => {
            let property = &column.property;
            (
                &property.decimal_char,
                &property.group_char,
                property.with_text,
            )
        }
        _ => return target_text_expr,
    };

    let source_text_expr = normalize_number_text(
        mapping.source.clone().cast(DataType::String),
        decimal_char.as_deref(),
        group_char.as_deref(),
        with_text == Some(true),
    );
    when(
        source_text_expr
            .clone()
            .cast(DataType::Float64)
            .eq(mapping.target.clone().cast(DataType::Float64)),
    )
    .then(source_text_expr)
    .otherwise(target_text_expr)
}

/// Python's `format(Decimal(text), "f")`: `1e-05` becomes `0.00001`.
fn expand_exponent(text: &str) -> String {
    let Some((mantissa, exponent)) = text.split_once('e') else {
        return text.to_string();
    };
    let exponent: i64 = exponent.parse().unwrap_or(0);
    let (sign, mantissa) = match mantissa.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", mantissa),
    };
    let (integer, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{integer}{fraction}");
    let point = integer.len() as i64 + exponent;
    if point <= 0 {
        return format!("{sign}0.{}{digits}", "0".repeat((-point) as usize));
    }
    let point = point as usize;
    if point >= digits.len() {
        return format!("{sign}{digits}{}", "0".repeat(point - digits.len()));
    }
    format!("{sign}{}.{}", &digits[..point], &digits[point..])
}
