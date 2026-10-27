use fairspec_metadata::TimeColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, get_utc_datetime_type, reject_leap_second};

const DEFAULT_FORMAT: &str = "%H:%M:%S%.fZ";
const DEFAULT_PARSE_FORMAT: &str = "%H:%M:%S%.f%#z";
const DEFAULT_PATTERN: &str = r"^\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$";

pub fn parse_time_column(column: &TimeColumn, column_expr: Expr) -> Expr {
    let format = column.property.temporal_format.as_deref();
    let text_expr = match format {
        Some(_) => column_expr.clone(),
        None => column_expr.clone().str().extract(lit(DEFAULT_PATTERN), 0),
    };
    let value_expr = concat_str([lit("1970-01-01T"), text_expr], "", false)
        .str()
        .strptime(
            get_utc_datetime_type(),
            create_strptime_options(&format!(
                "%Y-%m-%dT{}",
                format.unwrap_or(DEFAULT_PARSE_FORMAT)
            )),
            lit("raise"),
        )
        .cast(DataType::Time);
    let value_expr = match format {
        Some(format) => reject_leap_second(column_expr, value_expr, format),
        None => value_expr,
    };
    value_expr.alias(column.name.as_str())
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
