use fairspec_metadata::UnknownColumn;
use polars::prelude::*;

pub fn stringify_unknown_column(_column: &UnknownColumn, column_expr: Expr) -> Expr {
    column_expr.cast(DataType::String)
}
