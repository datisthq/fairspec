use fairspec_metadata::{CellError, CellMultipleOfError, Column};
use polars::prelude::*;
use serde_json::Value;

use super::check::ColumnCheck;
use crate::helpers::general::format_python_float;
use crate::models::cell::CellMapping;

const DECIMAL: DataType = DataType::Decimal(38, 18);
const MAX_DECIMAL_QUOTIENT: f64 = 1e18;

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
    let float_expr = mapping.target.clone().cast(DataType::Float64);
    let is_float_error_expr = (float_expr.clone() % lit(divisor_number)).eq(lit(0)).not();
    if divisor_number < 1.0 / MAX_DECIMAL_QUOTIENT || divisor_number >= MAX_DECIMAL_QUOTIENT {
        return Some(ColumnCheck {
            is_error_expr: is_float_error_expr,
            error_template,
        });
    }

    // Binary floats make 99.99 % 0.01 non-zero, so the shortest text of the value is
    // divided as a decimal; a quotient past the decimal range would abort the query
    let is_decimal_expr = (float_expr / lit(divisor_number))
        .abs()
        .lt(lit(MAX_DECIMAL_QUOTIENT));
    let value_expr = when(is_decimal_expr.clone())
        .then(mapping.target.clone().cast(DataType::String))
        .otherwise(lit("0"))
        .cast(DECIMAL);

    let divisor = match multiple_of.as_i64() {
        Some(integer) => integer.to_string(),
        None => expand_exponent(&format_python_float(divisor_number)),
    };
    let quotient_expr = value_expr / lit(divisor).cast(DECIMAL);
    let is_error_expr = when(is_decimal_expr)
        .then(quotient_expr.clone().neq(quotient_expr.floor()))
        .otherwise(is_float_error_expr);

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
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
