import { getBasePropertyType } from "@fairspec/metadata"
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

export function denarrowColumn(mapping: ColumnMapping, columnExpr: pl.Expr) {
  const variant = mapping.source.type.variant

  if (mapping.target.type === "categorical") {
    if (ALPHANUMERIC_VARIANTS.includes(variant)) {
      const { values, labels } = getCategoricalValuesAndLabels(mapping.target)

      const polarsType =
        getBasePropertyType(mapping.target.property.type) === "string"
          ? pl.String
          : pl.Int64

      return values.length
        ? columnExpr.replaceStrict(labels, values, pl.lit(null), polarsType)
        : columnExpr
    }
  }

  return columnExpr
}
