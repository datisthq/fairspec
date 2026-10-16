import type { CellMinItemsError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellMinItems(column: Column, mapping: CellMapping) {
  if (column.type !== "list") return undefined

  const minItems = column.property.minItems
  if (minItems === undefined) return undefined

  const isErrorExpr = mapping.target.lst.lengths().lt(minItems)

  const errorTemplate: CellMinItemsError = {
    type: "cell/minItems",
    columnName: column.name,
    minItems: minItems,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
