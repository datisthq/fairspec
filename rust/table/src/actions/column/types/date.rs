use fairspec_metadata::DateColumn;
use polars::prelude::*;

use super::temporal::create_strptime_options;

const DEFAULT_FORMAT: &str = "%Y-%m-%d";

pub fn parse_date_column(column: &DateColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    column_expr.str().strptime(
        DataType::Date,
        create_strptime_options(format),
        lit("raise"),
    )
}

pub fn stringify_date_column(column: &DateColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    column_expr.dt().strftime(format)
}
