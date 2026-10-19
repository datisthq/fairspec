import type { CellMultipleOfError, Column } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { CellMapping } from "../../../models/cell.ts"

const DECIMAL = pl.Decimal(38, 18)
const MAX_DECIMAL_QUOTIENT = 1e18

export function checkCellMultipleOf(column: Column, mapping: CellMapping) {
  const property = column.property
  if (!("multipleOf" in property)) return undefined

  const multipleOf = property.multipleOf
  if (multipleOf === undefined) return undefined

  const errorTemplate: CellMultipleOfError = {
    type: "cell/multipleOf",
    columnName: column.name,
    multipleOf,
    rowNumber: 0,
    cell: "",
  }

  const floatExpr = mapping.target.cast(pl.Float64)
  const isFloatErrorExpr = floatExpr.modulo(multipleOf).eq(0).not()
  if (multipleOf < 1 / MAX_DECIMAL_QUOTIENT || multipleOf >= MAX_DECIMAL_QUOTIENT) {
    return { isErrorExpr: isFloatErrorExpr, errorTemplate }
  }

  // Binary floats make 99.99 % 0.01 non-zero, so the shortest text of the value is
  // divided as a decimal; a quotient past the decimal range would abort the query
  const isDecimalExpr = floatExpr.divideBy(multipleOf).abs().lt(MAX_DECIMAL_QUOTIENT)
  const valueExpr = pl
    .when(isDecimalExpr)
    .then(mapping.target.cast(pl.String))
    .otherwise(pl.lit("0"))
    .cast(DECIMAL)

  const quotientExpr = valueExpr.divideBy(pl.lit(String(multipleOf)).cast(DECIMAL))
  const isErrorExpr = pl
    .when(isDecimalExpr)
    .then(quotientExpr.neq(quotientExpr.floor()))
    .otherwise(isFloatErrorExpr)

  return { isErrorExpr, errorTemplate }
}
