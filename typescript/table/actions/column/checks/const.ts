import type { CellConstError, Column } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { CellMapping } from "../../../models/cell.ts"

export function checkCellConst(column: Column, mapping: CellMapping) {
  const constConstraint = column.property.const
  if (!constConstraint) return undefined

  const primitiveConstConstraint =
    typeof constConstraint === "object"
      ? JSON.stringify(constConstraint)
      : constConstraint

  const isErrorExpr = mapping.target.eq(pl.lit(primitiveConstConstraint)).not()

  const errorTemplate: CellConstError = {
    type: "cell/const",
    columnName: column.name,
    const: primitiveConstConstraint.toString(),
    rowNumber: 0,
    cell: "",
  }

  return { isErrorExpr, errorTemplate }
}
