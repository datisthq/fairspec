use fairspec_metadata::CellError;
use polars::prelude::Expr;

/// A per-cell check: an expression true for a failing cell, and the error to report,
/// whose `rowNumber` and `cell` are filled in per failing row.
#[derive(Debug, Clone)]
pub struct ColumnCheck {
    pub is_error_expr: Expr,
    pub error_template: CellError,
}
