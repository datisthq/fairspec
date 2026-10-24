use polars::prelude::Expr;

#[derive(Debug, Clone)]
pub struct CellMapping {
    pub source: Expr,
    pub target: Expr,
}
