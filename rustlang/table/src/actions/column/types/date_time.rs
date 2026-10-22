use fairspec_metadata::DateTimeColumn;
use polars::prelude::*;

use super::temporal::{create_strptime_options, get_utc_datetime_type};

const DEFAULT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.fZ";
const DEFAULT_PARSE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%#z";
const DEFAULT_PATTERN: &str = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$";

pub fn parse_date_time_column(column: &DateTimeColumn, column_expr: Expr) -> Expr {
    if let Some(format) = &column.property.temporal_format {
        return column_expr.str().strptime(
            get_utc_datetime_type(),
            create_strptime_options(format),
            lit("raise"),
        );
    }

    // %#z also reads `+02` and `+0200`, which the default form does not allow
    let text_expr = column_expr.str().extract(lit(DEFAULT_PATTERN), 0);
    text_expr.str().strptime(
        get_utc_datetime_type(),
        create_strptime_options(DEFAULT_PARSE_FORMAT),
        lit("raise"),
    )
}

pub fn stringify_date_time_column(column: &DateTimeColumn, column_expr: Expr) -> Expr {
    let format = column
        .property
        .temporal_format
        .as_deref()
        .unwrap_or(DEFAULT_FORMAT);
    column_expr.dt().strftime(format)
}
