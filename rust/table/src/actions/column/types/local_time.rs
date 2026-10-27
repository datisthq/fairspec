use fairspec_metadata::LocalTimeColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, reject_leap_second};

const DEFAULT_FORMAT: &str = "%H:%M:%S%.f";
const DEFAULT_PATTERN: &str = r"^\d{2}:\d{2}:\d{2}(\.\d+)?$";

pub fn parse_local_time_column(column: &LocalTimeColumn, column_expr: Expr) -> Expr {
    let format = column.property.temporal_format.as_deref();
    let text_expr = match format {
        Some(_) => column_expr.clone(),
        None => column_expr.clone().str().extract(lit(DEFAULT_PATTERN), 0),
    };
    let value_expr = concat_str([lit("1970-01-01T"), text_expr], "", false)
        .str()
        .strptime(
            DataType::Datetime(TimeUnit::Microseconds, None),
            create_strptime_options(&format!("%Y-%m-%dT{}", format.unwrap_or(DEFAULT_FORMAT))),
            lit("raise"),
        )
        .cast(DataType::Time);
    let value_expr = match format {
        Some(format) => reject_leap_second(column_expr, value_expr, format),
        None => value_expr,
    };
    value_expr.alias(column.name.as_str())
}

pub fn stringify_local_time_column(column: &LocalTimeColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    column_expr.dt().strftime(format)
}
