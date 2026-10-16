import type { HexColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { z } from "zod"

export function parseHexColumn(column: HexColumn, columnExpr: pl.Expr) {
  return pl
    .when(columnExpr.str.contains(z.regexes.hex))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
