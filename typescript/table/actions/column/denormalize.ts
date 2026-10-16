import * as pl from "nodejs-polars"
import type { ColumnMapping, DenormalizeColumnOptions } from "../../models/column.ts"
import { denarrowColumn } from "./denarrow.ts"
import { desubstituteColumn } from "./desubstitute.ts"
import { stringifyColumn } from "./stringify.ts"

export function denormalizeColumn(
  mapping: ColumnMapping,
  options?: DenormalizeColumnOptions,
) {
  let columnExpr = pl.col(mapping.source.name)

  if (!options?.nativeTypes?.includes(mapping.target.type)) {
    columnExpr = denarrowColumn(mapping, columnExpr)
    columnExpr = stringifyColumn(mapping, columnExpr)
  }

  columnExpr = desubstituteColumn(mapping, columnExpr, options)
  return columnExpr
}
