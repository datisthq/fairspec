import type { HexColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { HEX_REGEX } from "../../../settings.ts"

export function parseHexColumn(column: HexColumn, columnExpr: pl.Expr) {
  return pl
    .when(columnExpr.str.contains(HEX_REGEX))
    .then(columnExpr)
    .otherwise(pl.lit(null))
    .alias(column.name)
}
