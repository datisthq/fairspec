import type { BooleanColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

const DEFAULT_TRUE_VALUES = ["true", "True", "TRUE", "1"]
const DEFAULT_FALSE_VALUES = ["false", "False", "FALSE", "0"]

export function parseBooleanColumn(column: BooleanColumn, columnExpr: pl.Expr) {
  const trueValues = column.property.trueValues ?? DEFAULT_TRUE_VALUES
  const falseValues = column.property.falseValues ?? DEFAULT_FALSE_VALUES

  return pl
    .when(columnExpr.isIn(trueValues))
    .then(pl.lit(true))
    .when(columnExpr.isIn(falseValues))
    .then(pl.lit(false))
    .otherwise(pl.lit(null).cast(pl.Bool))
    .alias(column.name)
}

const DEFAULT_TRUE_VALUE = "true"
const DEFAULT_FALSE_VALUE = "false"

export function stringifyBooleanColumn(column: BooleanColumn, columnExpr: pl.Expr) {
  const trueValue = column.property.trueValues?.[0] ?? DEFAULT_TRUE_VALUE
  const falseValue = column.property.falseValues?.[0] ?? DEFAULT_FALSE_VALUE

  return pl
    .when(columnExpr.eq(pl.lit(true)))
    .then(pl.lit(trueValue))
    .otherwise(pl.lit(falseValue))
    .alias(column.name)
}
