use polars::prelude::*;

use super::narrow::narrow_column;
use super::parse::parse_column;
use super::substitute::substitute_column;
use crate::models::column::ColumnMapping;

/// The expression turning a source column into its schema type: missing values
/// substituted, then (unless `keep_type`) parsed and narrowed.
pub fn normalize_column(mapping: &ColumnMapping, keep_type: bool) -> Expr {
    let mut column_expr = substitute_column(mapping, col(mapping.source.name.as_str()));
    if !keep_type {
        column_expr = parse_column(mapping, column_expr);
        column_expr = narrow_column(mapping, column_expr);
    }
    column_expr.alias(mapping.target.name())
}
