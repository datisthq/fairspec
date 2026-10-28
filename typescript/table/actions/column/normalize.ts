import * as pl from "nodejs-polars"
import type { ColumnMapping } from "../../models/column.ts"
import { narrowColumn } from "./narrow.ts"
import { parseColumn } from "./parse.ts"
import { substituteColumn } from "./substitute.ts"

export function normalizeColumn(
  mapping: ColumnMapping,
  options?: { keepType?: boolean; isPlain?: boolean },
) {
  let columnExpr = pl.col(mapping.source.name)
  columnExpr = substituteColumn(mapping, columnExpr)

  if (!options?.keepType) {
    columnExpr = parseColumn(mapping, columnExpr, { isPlain: options?.isPlain })
    columnExpr = narrowColumn(mapping, columnExpr)
  }

  return columnExpr.alias(mapping.target.name)
}
