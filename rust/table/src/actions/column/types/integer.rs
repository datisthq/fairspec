use fairspec_metadata::IntegerColumn;
use polars::prelude::*;

use crate::actions::column::_shared::text::strip_number_text;

pub fn parse_integer_column(column: &IntegerColumn, column_expr: Expr) -> Expr {
    let mut column_expr = column_expr;
    if column.property.base.with_text == Some(true) {
        column_expr = strip_number_text(column_expr, None);
    }
    if let Some(group_char) = column
        .property
        .base
        .group_char
        .as_deref()
        .filter(|c| !c.is_empty())
    {
        column_expr = column_expr
            .str()
            .replace_all(lit(regex::escape(group_char)), lit(""), false);
    }
    column_expr.cast(DataType::Int64)
}

pub fn stringify_integer_column(_column: &IntegerColumn, column_expr: Expr) -> Expr {
    column_expr.cast(DataType::String)
}
