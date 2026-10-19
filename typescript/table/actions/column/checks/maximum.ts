import type {
  CellExclusiveMaximumError,
  CellMaximumError,
  Column,
} from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"

const NUMERIC_COLUMN_TYPES: Column["type"][] = ["integer", "number", "decimal"]

export function createCheckCellMaximum(options?: { isExclusive?: boolean }) {
  return (column: Column, mapping: CellMapping) => {
    if (!NUMERIC_COLUMN_TYPES.includes(column.type)) return undefined

    const property = column.property
    if (!("maximum" in property) && !("exclusiveMaximum" in property)) return undefined

    const maximum = options?.isExclusive ? property.exclusiveMaximum : property.maximum
    if (maximum === undefined) return undefined

    const isErrorExpr = options?.isExclusive
      ? mapping.target.gtEq(maximum)
      : mapping.target.gt(maximum)

    const errorTemplate: CellMaximumError | CellExclusiveMaximumError = {
      type: options?.isExclusive ? "cell/exclusiveMaximum" : "cell/maximum",
      columnName: column.name,
      maximum: String(maximum),
      rowNumber: 0,
      cell: "",
    }

    return { isErrorExpr, errorTemplate }
  }
}
