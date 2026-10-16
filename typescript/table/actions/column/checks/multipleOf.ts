import type { CellMultipleOfError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellMultipleOf(column: Column, mapping: CellMapping) {
  const property = column.property
  if (!("multipleOf" in property)) return undefined

  const multipleOf = property.multipleOf
  if (multipleOf === undefined) return undefined

  const isErrorExpr = mapping.target.modulo(multipleOf).eq(0).not()

  const errorTemplate: CellMultipleOfError = {
    type: "cell/multipleOf",
    columnName: column.name,
    multipleOf,
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
