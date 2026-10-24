use fairspec_metadata::LocalDateTimeColumn;
use polars::prelude::*;

use super::temporal::create_strptime_options;

const DEFAULT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
const DEFAULT_PATTERN: &str = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?$";

pub fn parse_local_date_time_column(column: &LocalDateTimeColumn, column_expr: Expr) -> Expr {
    let data_type = DataType::Datetime(TimeUnit::Microseconds, None);
    if let Some(format) = &column.property.temporal_format {
        return column_expr.str().strptime(
            data_type,
            create_strptime_options(format),
            lit("raise"),
        );
    }

    let text_expr = column_expr.str().extract(lit(DEFAULT_PATTERN), 0);
    text_expr.str().strptime(
        data_type,
        create_strptime_options(DEFAULT_FORMAT),
        lit("raise"),
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
