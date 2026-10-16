import * as pl from "nodejs-polars"
import type { ColumnMapping, PolarsColumn } from "../../models/column.ts"

export function substituteColumn(mapping: ColumnMapping, columnExpr: pl.Expr) {
  // As we know what source polars column type is,
  // we can filter out incompatible missing values

  const missingValueType = getMissingValueType(mapping.source)
  if (!missingValueType) {
    return columnExpr
  }

  const flattenMissingValues = (mapping.target.property.missingValues ?? []).map(it =>
    typeof it === "object" ? it.value : it,
  )

  const compatibleMissingValues = flattenMissingValues.filter(
    value => typeof value === missingValueType,
  )

  if (!compatibleMissingValues.length) {
    return columnExpr
  }

  return pl
    .when(columnExpr.isIn(compatibleMissingValues))
    .then(pl.lit(null))
    .otherwise(columnExpr)
    .alias(mapping.target.name)
}

function getMissingValueType(polarsColumn: PolarsColumn) {
  const polarsType = polarsColumn.type

  if (polarsType.equals(pl.String)) return "string"
  if (polarsType.equals(pl.Int8)) return "number"
  if (polarsType.equals(pl.Int16)) return "number"
  if (polarsType.equals(pl.Int32)) return "number"
  if (polarsType.equals(pl.Int64)) return "number"
  if (polarsType.equals(pl.Float32)) return "number"
  if (polarsType.equals(pl.Float64)) return "number"

  return undefined
}
