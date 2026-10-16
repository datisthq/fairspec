import type { CellMissingError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellMissing(column: Column, mapping: CellMapping) {
  if (column.nullable) return undefined

  const isErrorExpr = mapping.target.isNull()

  const errorTemplate: CellMissingError = {
    type: "cell/missing",
    columnName: column.name,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
