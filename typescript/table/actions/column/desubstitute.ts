import type { Column } from "@fairspec/metadata"
import { getBasePropertyType } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { ColumnMapping, DenormalizeColumnOptions } from "../../models/column.ts"

export function desubstituteColumn(
  mapping: ColumnMapping,
  columnExpr: pl.Expr,
  options?: DenormalizeColumnOptions,
) {
  const missingValueType = getMissingValueType(mapping.target, options)
  if (!missingValueType) {
    return columnExpr
  }

  const flattenMissingValues = (mapping.target.property.missingValues ?? []).map(it =>
    typeof it === "object" ? it.value : it,
  )

  const compatibleMissingValue = flattenMissingValues?.filter(
    value => typeof value === missingValueType,
  )[0]

  if (compatibleMissingValue === undefined) {
    return columnExpr
  }

  return pl
    .when(columnExpr.isNull())
    .then(pl.lit(compatibleMissingValue))
    .otherwise(columnExpr)
    .alias(mapping.target.name)
}

// TODD: Improve this initial implementation

function getMissingValueType(column: Column, options?: DenormalizeColumnOptions) {
  const baseType = getBasePropertyType(column.property.type ?? "null")

  if (baseType === "string") {
    return "string"
  }

  if (baseType === "integer" || baseType === "number") {
    return options?.nativeTypes?.includes(baseType) ? "number" : "string"
  }

  return undefined
}
