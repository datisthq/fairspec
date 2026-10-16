import type { Base64Column } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { z } from "zod"

export function parseBase64Column(column: Base64Column, columnExpr: pl.Expr) {
  return pl
    .when(columnExpr.str.contains(z.regexes.base64))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
