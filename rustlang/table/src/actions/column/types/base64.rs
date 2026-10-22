use fairspec_metadata::Base64Column;
use polars::prelude::*;

use crate::settings::BASE64_REGEX;

pub fn parse_base64_column(column: &Base64Column, column_expr: Expr) -> Expr {
    when(column_expr.clone().str().contains(lit(BASE64_REGEX), true))
        .then(column_expr)
        .otherwise(lit(NULL))
        .alias(column.name.as_str())
}
