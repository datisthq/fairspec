import type { TimeColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { keepExactDefaultTemporal, keepExactTemporal } from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%H:%M:%S"
const DEFAULT_PATTERN = "^\\d{2}:\\d{2}:\\d{2}$"

export function parseTimeColumn(column: TimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat ?? DEFAULT_FORMAT
  const textExpr = column.property.temporalFormat
    ? columnExpr
    : columnExpr.str.extract(DEFAULT_PATTERN, 0)

  const valueExpr = pl.pl
    .concatString([pl.pl.lit("1970-01-01T"), textExpr], "")
    .str.strptime(pl.Datetime, `%Y-%m-%dT${format}`)
    .cast(pl.Time)

  const keepExact = column.property.temporalFormat
    ? keepExactTemporal
    : keepExactDefaultTemporal
  return keepExact(textExpr, valueExpr, format).alias(column.name)
}

export function stringifyTimeColumn(column: TimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat ?? DEFAULT_FORMAT

  return columnExpr.dt.strftime(format)
}
