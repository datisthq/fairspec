use fairspec_metadata::DateColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, reject_leap_second};

const DEFAULT_FORMAT: &str = "%Y-%m-%d";
const DEFAULT_PATTERN: &str = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$";

pub fn parse_date_column(column: &DateColumn, column_expr: Expr) -> Expr {
    if let Some(format) = &column.property.temporal_format {
        let value_expr = column_expr.clone().str().strptime(
            DataType::Date,
            create_strptime_options(format),
            lit("raise"),
        );
        return reject_leap_second(column_expr, value_expr, format);
    }

    parse_date_text(column_expr)
}

pub fn parse_date_text(text_expr: Expr) -> Expr {
    text_expr
        .str()
        .extract(lit(DEFAULT_PATTERN), 0)
        .str()
        .strptime(
            DataType::Date,
            create_strptime_options(DEFAULT_FORMAT),
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
