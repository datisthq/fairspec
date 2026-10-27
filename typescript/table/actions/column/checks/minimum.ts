import type {
  CellExclusiveMinimumError,
  CellMinimumError,
  Column,
} from "@fairspec/metadata"
import type { CellMapping } from "../../../models/cell.ts"
import { getNumericValue } from "./-shared/value.ts"
import { createBoundExpr } from "./maximum.ts"

export function createCheckCellMinimum(options?: { isExclusive?: boolean }) {
  return (column: Column, mapping: CellMapping) => {
    const valueExpr = getNumericValue(column, mapping)
    if (!valueExpr) return undefined

    const property = column.property
    if (!("minimum" in property) && !("exclusiveMinimum" in property)) return undefined

    const minimum = options?.isExclusive ? property.exclusiveMinimum : property.minimum
    if (typeof minimum !== "number") return undefined

    const minimumExpr = createBoundExpr(minimum)
    const isErrorExpr = options?.isExclusive
      ? valueExpr.ltEq(minimumExpr)
      : valueExpr.lt(minimumExpr)

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
