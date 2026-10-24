use fairspec_metadata::DecimalColumn;
use polars::prelude::*;

use crate::actions::column::_shared::text::strip_number_text;

// polars 2's default precision for a scale-only Decimal
const DECIMAL_PRECISION: usize = 38;
const DECIMAL_SCALE: usize = 18;

pub fn parse_decimal_column(column: &DecimalColumn, column_expr: Expr) -> Expr {
    let property = &column.property;
    normalize_number_text(
        column_expr,
        property.decimal_char.as_deref(),
        property.group_char.as_deref(),
        property.with_text == Some(true),
    )
    .cast(DataType::Decimal(DECIMAL_PRECISION, DECIMAL_SCALE))
}

pub fn stringify_decimal_column(_column: &DecimalColumn, column_expr: Expr) -> Expr {
    column_expr.cast(DataType::String)
}

/// Rewrites number text to the `1234.5` form a cast reads.
pub(crate) fn normalize_number_text(
    column_expr: Expr,
    decimal_char: Option<&str>,
    group_char: Option<&str>,
    with_text: bool,
) -> Expr {
    let decimal_char = decimal_char.filter(|c| !c.is_empty()).unwrap_or(".");
    let group_char = group_char.unwrap_or("");
    let mut column_expr = column_expr;
    if with_text {
        column_expr = strip_number_text(column_expr, Some(decimal_char));
    }

    if group_char == "." && decimal_char == "," {
        column_expr = column_expr
            .str()
            .replace_all(lit(","), lit("###DECIMAL###"), false)
            .str()
            .replace_all(lit(r"\."), lit(""), false)
            .str()
            .replace_all(lit("###DECIMAL###"), lit("."), false);
    } else {
        if !group_char.is_empty() {
            column_expr =
                column_expr
                    .str()
                    .replace_all(lit(regex::escape(group_char)), lit(""), false);
        }
        if decimal_char != "." {
            column_expr =
                column_expr
                    .str()
                    .replace_all(lit(regex::escape(decimal_char)), lit("."), false);
        }
    }
    column_expr
}
