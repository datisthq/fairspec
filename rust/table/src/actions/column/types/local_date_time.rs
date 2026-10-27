use fairspec_metadata::LocalDateTimeColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, parse_default_temporal, reject_leap_second};

const DEFAULT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
const DEFAULT_PATTERN: &str =
    r"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?$";

pub fn parse_local_date_time_column(column: &LocalDateTimeColumn, column_expr: Expr) -> Expr {
    let data_type = DataType::Datetime(TimeUnit::Microseconds, None);
    if let Some(format) = &column.property.temporal_format {
        let value_expr = column_expr.clone().str().strptime(
            data_type,
            create_strptime_options(format),
            lit("raise"),
        );
        return reject_leap_second(column_expr, value_expr, format);
    }

    parse_local_date_time_text(column_expr).alias(column.name.as_str())
}

pub fn parse_local_date_time_text(text_expr: Expr) -> Expr {
    parse_default_temporal(
        text_expr,
        DEFAULT_PATTERN,
        DEFAULT_FORMAT,
        DataType::Datetime(TimeUnit::Microseconds, None),
        "",
    )
}

pub fn stringify_local_date_time_column(column: &LocalDateTimeColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    column_expr.dt().strftime(format)
}
