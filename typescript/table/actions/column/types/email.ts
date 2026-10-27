import type { EmailColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { EMAIL_REGEX } from "../../../settings.ts"

export function parseEmailColumn(column: EmailColumn, columnExpr: pl.Expr) {
  return pl
    .when(columnExpr.str.contains(EMAIL_REGEX))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
