import type { UnknownColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

// TODO: review

export function stringifyUnkonwnColumn(_column: UnknownColumn, columnExpr: pl.Expr) {
  return columnExpr.cast(pl.String)
}
