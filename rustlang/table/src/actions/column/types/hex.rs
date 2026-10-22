use fairspec_metadata::HexColumn;
use polars::prelude::*;

use crate::settings::HEX_REGEX;

pub fn parse_hex_column(column: &HexColumn, column_expr: Expr) -> Expr {
    when(column_expr.clone().str().contains(lit(HEX_REGEX), true))
        .then(column_expr)
        .otherwise(lit(NULL))
        .alias(column.name.as_str())
}
