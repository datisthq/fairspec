use fairspec_metadata::DateColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, reject_leap_second};

const DEFAULT_FORMAT: &str = "%Y-%m-%d";
const DEFAULT_PATTERN: &str = r"^\d{4}-\d{2}-\d{2}$";

pub fn parse_date_column(column: &DateColumn, column_expr: Expr) -> Expr {
    if let Some(format) = &column.property.temporal_format {
        let value_expr = column_expr.clone().str().strptime(
            DataType::Date,
            create_strptime_options(format),
            lit("raise"),
        );
        return reject_leap_second(column_expr, value_expr, format);
    }

    let text_expr = column_expr.str().extract(lit(DEFAULT_PATTERN), 0);
    text_expr.str().strptime(
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
