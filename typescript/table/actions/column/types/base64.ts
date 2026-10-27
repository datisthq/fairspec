import type { Base64Column } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { BASE64_REGEX } from "../../../settings.ts"

export function parseBase64Column(column: Base64Column, columnExpr: pl.Expr) {
  return pl
    .when(columnExpr.str.contains(BASE64_REGEX))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
