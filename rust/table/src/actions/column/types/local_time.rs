use fairspec_metadata::LocalTimeColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, parse_default_temporal, reject_leap_second};

const DEFAULT_FORMAT: &str = "%H:%M:%S%.f";
const DEFAULT_PATTERN: &str = r"^[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?$";
const DATE_PREFIX: &str = "1970-01-01T";

pub fn parse_local_time_column(column: &LocalTimeColumn, column_expr: Expr) -> Expr {
    let Some(format) = column.property.temporal_format.as_deref() else {
        return parse_local_time_text(column_expr).alias(column.name.as_str());
    };
    let value_expr = concat_str([lit(DATE_PREFIX), column_expr.clone()], "", false)
        .str()
        .strptime(
            DataType::Datetime(TimeUnit::Microseconds, None),
            create_strptime_options(&format!("%Y-%m-%dT{format}")),
            lit("raise"),
        )
        .cast(DataType::Time);
    reject_leap_second(column_expr, value_expr, format).alias(column.name.as_str())
}

pub fn parse_local_time_text(text_expr: Expr) -> Expr {
    parse_default_temporal(
        text_expr,
        DEFAULT_PATTERN,
        &format!("%Y-%m-%dT{DEFAULT_FORMAT}"),
        DataType::Datetime(TimeUnit::Microseconds, None),
        DATE_PREFIX,
    )
    .cast(DataType::Time)
}

pub fn stringify_local_time_column(column: &LocalTimeColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    column_expr.dt().strftime(format)
}
