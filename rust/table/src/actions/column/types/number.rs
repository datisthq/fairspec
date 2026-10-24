use fairspec_metadata::NumberColumn;
use polars::prelude::*;

use super::decimal::normalize_number_text;

pub fn parse_number_column(column: &NumberColumn, column_expr: Expr) -> Expr {
    let property = &column.property.base;
    normalize_number_text(
        column_expr,
        property.decimal_char.as_deref(),
        property.group_char.as_deref(),
        property.with_text == Some(true),
    )
    .cast(DataType::Float64)
}

pub fn stringify_number_column(_column: &NumberColumn, column_expr: Expr) -> Expr {
    column_expr.cast(DataType::String)
}
