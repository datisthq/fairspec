import type { CellEnumError, Column } from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellEnum(column: Column, mapping: CellMapping) {
  const enumConstraint = column.property.enum
  if (!enumConstraint) return undefined

  const primitiveEnumConstraint = enumConstraint.map(item =>
    typeof item === "object" ? JSON.stringify(item) : item,
  )

  const isErrorExpr = mapping.target.isIn(primitiveEnumConstraint).not()

  const errorTemplate: CellEnumError = {
    type: "cell/enum",
    columnName: column.name,
    enum: enumConstraint.map(String),
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
