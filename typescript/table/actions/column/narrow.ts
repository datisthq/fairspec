import * as pl from "nodejs-polars"
import { getCategoricalValuesAndLabels } from "../../helpers/column.ts"
import type { ColumnMapping } from "../../models/column.ts"

// TODO: Remove duplication narrow/denarrow
const INTEGER_VARIANTS = ["Int8", "Int16", "Int32", "Int64"]
const NUMBER_VARIANTS = ["Float32", "Float64"]
const STRING_VARIANTS = ["String", "Categorical"]
const ALPHANUMERIC_VARIANTS = [
  ...INTEGER_VARIANTS,
  ...NUMBER_VARIANTS,
  ...STRING_VARIANTS,
]

export function narrowColumn(mapping: ColumnMapping, columnExpr: pl.Expr) {
  const variant = mapping.source.type.variant

  if (mapping.target.type === "boolean") {
    if (INTEGER_VARIANTS.includes(variant)) {
      columnExpr = pl
        .when(columnExpr.eq(1))
        .then(pl.lit(true))
        .when(columnExpr.eq(0))
        .then(pl.lit(false))
        .otherwise(pl.lit(null))
    }
  }

  if (mapping.target.type === "integer") {
    if (NUMBER_VARIANTS.includes(variant)) {
      columnExpr = pl
        .when(columnExpr.eq(columnExpr.round(0)))
        .then(columnExpr.cast(pl.Int64))
        .otherwise(pl.lit(null))
    }
  }

  if (mapping.target.type === "categorical") {
    if (ALPHANUMERIC_VARIANTS.includes(variant)) {
      const { values, labels } = getCategoricalValuesAndLabels(mapping.target)

      return values.length
        ? columnExpr.replaceStrict(values, labels, pl.lit(null), pl.Categorical)
        : columnExpr
    }
  }

  return columnExpr
}
