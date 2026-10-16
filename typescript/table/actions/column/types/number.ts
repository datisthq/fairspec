import type { NumberColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

export function parseNumberColumn(column: NumberColumn, columnExpr: pl.Expr) {
  // Extract the decimal and group characters
  const decimalChar = column.property.decimalChar ?? "."
  const groupChar = column.property.groupChar ?? ""
  const withText = column.property.withText

  // Special case handling for European number format where "." is group and "," is decimal
  if (groupChar === "." && decimalChar === ",") {
    // First temporarily replace the decimal comma with a placeholder
    columnExpr = columnExpr.str.replaceAll(",", "###DECIMAL###")
    // Remove the group dots
    columnExpr = columnExpr.str.replaceAll("\\.", "")
    // Replace the placeholder with an actual decimal point
    columnExpr = columnExpr.str.replaceAll("###DECIMAL###", ".")
  } else {
    // Standard case: first remove group characters
    if (groupChar) {
      // Escape special characters for regex
      const escapedGroupChar = groupChar.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
      columnExpr = columnExpr.str.replaceAll(escapedGroupChar, "")
    }

    // Then handle decimal character
    if (decimalChar && decimalChar !== ".") {
      columnExpr = columnExpr.str.replaceAll(decimalChar, ".")
    }
  }

  // Handle numbers with text (with currency symbols, percent signs, etc.)
  if (withText) {
    // Allowing "e" for scientific notation
    columnExpr = columnExpr.str.replaceAll("[^\\d\\-.e]", "")
  }

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
