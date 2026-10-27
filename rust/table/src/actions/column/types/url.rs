use fairspec_metadata::UrlColumn;
use polars::prelude::*;

use crate::settings::URL_REGEX;

pub fn parse_url_column(column: &UrlColumn, column_expr: Expr) -> Expr {
    when(
        column_expr
            .clone()
            .str()
            .contains(lit(URL_REGEX.as_str()), true),
    )
    .then(column_expr)
    .otherwise(lit(NULL))
    .alias(column.name.as_str())
}
