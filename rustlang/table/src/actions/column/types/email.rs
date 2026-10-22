use fairspec_metadata::EmailColumn;
use polars::prelude::*;

use crate::settings::RFC5322_EMAIL_REGEX;

pub fn parse_email_column(column: &EmailColumn, column_expr: Expr) -> Expr {
    when(
        column_expr
            .clone()
            .str()
            .contains(lit(RFC5322_EMAIL_REGEX), true),
    )
    .then(column_expr)
    .otherwise(lit(NULL))
    .alias(column.name.as_str())
}
