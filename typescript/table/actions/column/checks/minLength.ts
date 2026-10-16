import type { CellMinLengthError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellMinLength(column: Column, mapping: CellMapping) {
  const property = column.property
  if (!("minLength" in property)) return undefined

  const minLength = property.minLength
  if (!minLength) return undefined

  // For string-based columns we test against the source polars column
  const isErrorExpr = mapping.source.str.lengths().lt(minLength)

  const errorTemplate: CellMinLengthError = {
    type: "cell/minLength",
    columnName: column.name,
    minLength: minLength,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
