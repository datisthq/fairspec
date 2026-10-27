use fairspec_metadata::NumberColumn;
use polars::prelude::*;

use super::decimal::normalize_number_text;

pub const NUMBER_PATTERN: &str = r"^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?$";

pub fn parse_number_column(column: &NumberColumn, column_expr: Expr) -> Expr {
    let property = &column.property.base;
    let with_text = property.with_text == Some(true);
    let mut column_expr = normalize_number_text(
        column_expr,
        property.decimal_char.as_deref(),
        property.group_char.as_deref(),
        with_text,
    );

    // withText keeps a decimal point before the first digit as part of the number
    if with_text {
        column_expr = column_expr
            .str()
            .replace(lit(r"^(-?)\."), lit("${1}0."), false);
    }

    parse_number_text(column_expr)
}

pub fn stringify_number_column(_column: &NumberColumn, column_expr: Expr) -> Expr {
    column_expr.cast(DataType::String)
}

/// Read a JSON number as a Float64, or null when the text is not one
pub fn parse_number_text(text_expr: Expr) -> Expr {
    when(text_expr.clone().str().contains(lit(NUMBER_PATTERN), false))
        .then(text_expr)
        .otherwise(lit(NULL))
        .cast(DataType::Float64)
}
