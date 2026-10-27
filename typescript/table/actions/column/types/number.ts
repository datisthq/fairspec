import type { NumberColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { normalizeNumberText } from "./decimal.ts"

export const NUMBER_PATTERN = "^-?(?:0|[1-9][0-9]*)(?:\\.[0-9]+)?(?:[eE][+-]?[0-9]+)?$"

export function parseNumberColumn(column: NumberColumn, columnExpr: pl.Expr) {
  columnExpr = normalizeNumberText(column, columnExpr)

  // withText keeps a decimal point before the first digit as part of the number
  if (column.property.withText) {
    columnExpr = columnExpr.str.replace("^(-?)\\.", "${1}0.")
  }

  return parseNumberText(columnExpr)
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

/**
 * Read a JSON number as a Float64, or null when the text is not one
 */
export function parseNumberText(textExpr: pl.Expr) {
  return pl
    .when(textExpr.str.contains(NUMBER_PATTERN))
    .then(textExpr)
    .otherwise(pl.lit(null))
    .cast(pl.Float64)
}
