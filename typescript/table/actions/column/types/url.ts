import type { UrlColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { URL_REGEX } from "../../../settings.ts"

export function parseUrlColumn(column: UrlColumn, columnExpr: pl.Expr) {
  return pl
    .when(columnExpr.str.contains(URL_REGEX))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
