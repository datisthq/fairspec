import type { CellMaxLengthError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellMaxLength(column: Column, mapping: CellMapping) {
  const property = column.property
  if (!("maxLength" in property)) return undefined

  const maxLength = property.maxLength
  if (!maxLength) return undefined

  // For string-based columns we test against the source polars column
  const isErrorExpr = mapping.source.str.lengths().gt(maxLength)

  const errorTemplate: CellMaxLengthError = {
    type: "cell/maxLength",
    columnName: column.name,
    maxLength: maxLength,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
