import type { UrlColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

export function parseUrlColumn(column: UrlColumn, columnExpr: pl.Expr) {
  // TODO: Update after fixed?
  // https://github.com/colinhacks/zod/pull/3497
  const regex = /^https?:\/\/.+/

  return pl
    .when(columnExpr.str.contains(regex))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
