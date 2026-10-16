import type { CellPatternError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellPattern(column: Column, mapping: CellMapping) {
  const property = column.property
  if (!("pattern" in property)) return undefined

  const pattern = property.pattern
  if (!pattern) return undefined

  // For string-based columns we test against the source polars column
  const isErrorExpr = mapping.source.str.contains(pattern).not()

  const errorTemplate: CellPatternError = {
    type: "cell/pattern",
    columnName: column.name,
    pattern: pattern,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
