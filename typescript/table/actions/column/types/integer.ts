import type { IntegerColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

export function parseIntegerColumn(column: IntegerColumn, columnExpr: pl.Expr) {
  const groupChar = column.property.groupChar
  const withText = column.property.withText

  // Handle non-bare numbers (with currency symbols, percent signs, etc.)
  if (withText) {
    // Preserve the minus sign when removing leading characters
    columnExpr = columnExpr.str.replaceAll("^[^\\d\\-]+", "")
    columnExpr = columnExpr.str.replaceAll("[^\\d\\-]+$", "")
  }

  // Handle group character (thousands separator)
  if (groupChar) {
    // Escape special characters for regex
    const escapedGroupChar = groupChar.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
    columnExpr = columnExpr.str.replaceAll(escapedGroupChar, "")
  }

  // Cast to int64 (will handle values up to 2^63-1)
  columnExpr = columnExpr.cast(pl.Int64)
  return columnExpr
}

export function stringifyIntegerColumn(_column: IntegerColumn, columnExpr: pl.Expr) {
  // Convert to string
  columnExpr = columnExpr.cast(pl.String)

  //const groupChar = column.groupChar
  //const bareNumber = column.bareNumber

  // TODO: Add group character formatting (thousands separator) when needed
  // TODO: Add non-bare number formatting (currency symbols, etc.) when needed

  return columnExpr
}
