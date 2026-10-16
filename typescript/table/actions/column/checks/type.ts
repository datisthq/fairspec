import type { CellTypeError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellType(column: Column, mapping: CellMapping) {
  const isErrorExpr = mapping.source.isNotNull().and(mapping.target.isNull())

  const errorTemplate: CellTypeError = {
    type: "cell/type",
    columnName: column.name,
    columnType: column.type,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
