import type { NumberColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { normalizeNumberText } from "./decimal.ts"

export function parseNumberColumn(column: NumberColumn, columnExpr: pl.Expr) {
  columnExpr = normalizeNumberText(column, columnExpr)

  // Cast to float64
  columnExpr = columnExpr.cast(pl.Float64)
  return columnExpr
}

export function stringifyNumberColumn(_column: NumberColumn, columnExpr: pl.Expr) {
  // Convert to string
  columnExpr = columnExpr.cast(pl.String)

  //const decimalChar = column.decimalChar ?? "."
  //const groupChar = column.groupChar ?? ""

  // TODO: Add decimal character formatting when needed
  // TODO: Add group character formatting (thousands separator) when needed
  // TODO: Add non-bare number formatting (currency symbols, etc.) when needed

  return columnExpr
}
