import type * as pl from "nodejs-polars"

export interface CellMapping {
  source: pl.Expr
  target: pl.Expr
}
