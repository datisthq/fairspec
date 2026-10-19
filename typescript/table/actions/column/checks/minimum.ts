import type {
  CellExclusiveMinimumError,
  CellMinimumError,
  Column,
} from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

const NUMERIC_COLUMN_TYPES: Column["type"][] = ["integer", "number", "decimal"]

export function createCheckCellMinimum(options?: { isExclusive?: boolean }) {
  return (column: Column, mapping: CellMapping) => {
    if (!NUMERIC_COLUMN_TYPES.includes(column.type)) return undefined

    const property = column.property
    if (!("minimum" in property) && !("exclusiveMinimum" in property)) return undefined

    const minimum = options?.isExclusive ? property.exclusiveMinimum : property.minimum
    if (minimum === undefined) return undefined

    const isErrorExpr = options?.isExclusive
      ? mapping.target.ltEq(minimum)
      : mapping.target.lt(minimum)

    const errorTemplate: CellMinimumError | CellExclusiveMinimumError = {
      type: options?.isExclusive ? "cell/exclusiveMinimum" : "cell/minimum",
      columnName: column.name,
      minimum: String(minimum),
      rowNumber: 0,
      cell: "",
    }

    return { isErrorExpr, errorTemplate }
  }
}
