import type {
  CellExclusiveMaximumError,
  CellMaximumError,
  Column,
} from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { CellMapping } from "../../../models/cell.ts"

const NUMERIC_COLUMN_TYPES: Column["type"][] = ["integer", "number", "decimal"]

export function createCheckCellMaximum(options?: { isExclusive?: boolean }) {
  return (column: Column, mapping: CellMapping) => {
    if (!NUMERIC_COLUMN_TYPES.includes(column.type)) return undefined

    const property = column.property
    if (!("maximum" in property) && !("exclusiveMaximum" in property)) return undefined

    const maximum = options?.isExclusive ? property.exclusiveMaximum : property.maximum
    if (typeof maximum !== "number") return undefined

    const maximumExpr = createBoundExpr(maximum)
    const isErrorExpr = options?.isExclusive
      ? mapping.target.gtEq(maximumExpr)
      : mapping.target.gt(maximumExpr)

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

// A JS number literal is a double, so an Int64 cell past 2^53 would compare rounded;
// a BigInt literal loses its sign in nodejs-polars, so the exact digits go in as text
export function createBoundExpr(bound: number) {
  if (Number.isInteger(bound) && Math.abs(bound) < 2 ** 63) {
    return pl.lit(BigInt(bound).toString()).cast(pl.Int64)
  }
  return pl.lit(bound)
}
