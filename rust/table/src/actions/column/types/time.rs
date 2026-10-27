use fairspec_metadata::TimeColumn;
use polars::prelude::*;

use super::temporal::{
    create_strptime_options, get_utc_datetime_type, parse_default_temporal, reject_leap_second,
};

const DEFAULT_FORMAT: &str = "%H:%M:%S%.fZ";
const DEFAULT_PARSE_FORMAT: &str = "%H:%M:%S%.f%#z";
const DEFAULT_PATTERN: &str =
    r"^[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$";
const DATE_PREFIX: &str = "1970-01-01T";

pub fn parse_time_column(column: &TimeColumn, column_expr: Expr) -> Expr {
    let Some(format) = column.property.temporal_format.as_deref() else {
        return parse_time_text(column_expr).alias(column.name.as_str());
    };
    let value_expr = concat_str([lit(DATE_PREFIX), column_expr.clone()], "", false)
        .str()
        .strptime(
            get_utc_datetime_type(),
            create_strptime_options(&format!("%Y-%m-%dT{format}")),
            lit("raise"),
        )
        .cast(DataType::Time);
    reject_leap_second(column_expr, value_expr, format).alias(column.name.as_str())
}

pub fn parse_time_text(text_expr: Expr) -> Expr {
    parse_default_temporal(
        text_expr,
        DEFAULT_PATTERN,
        &format!("%Y-%m-%dT{DEFAULT_PARSE_FORMAT}"),
        get_utc_datetime_type(),
        DATE_PREFIX,
    )
    .cast(DataType::Time)
}

pub fn stringify_time_column(column: &TimeColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    // A Time has no zone, and an offset specification only formats a zoned date-time
    lit(0)
        .cast(DataType::Date)
        .dt()
        .combine(column_expr, TimeUnit::Microseconds)
        .dt()
        .replace_time_zone(Some(TimeZone::UTC), lit("raise"), NonExistent::Raise)
        .dt()
        .strftime(format)
        .alias(column.name.as_str())
}
