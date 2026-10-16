import type { CellMaxItemsError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellMaxItems(column: Column, mapping: CellMapping) {
  if (column.type !== "list") return undefined

  const maxItems = column.property.maxItems
  if (maxItems === undefined) return undefined

  const isErrorExpr = mapping.target.lst.lengths().gt(maxItems)

  const errorTemplate: CellMaxItemsError = {
    type: "cell/maxItems",
    columnName: column.name,
    maxItems: maxItems,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
