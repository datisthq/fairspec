import type { EmailColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { z } from "zod"

export function parseEmailColumn(column: EmailColumn, columnExpr: pl.Expr) {
  return (
    pl
      // We use the RFC 5322 email regex as polars does not support look aheads
      .when(columnExpr.str.contains(z.regexes.rfc5322Email))
      .then(columnExpr)
      .otherwise(pl.lit(null))
      .alias(column.name)
  )
}
