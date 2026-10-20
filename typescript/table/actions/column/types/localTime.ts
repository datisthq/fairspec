import type { LocalTimeColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { parseDefaultLocalTemporal, parseLocalTemporal } from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%H:%M:%S"
const DEFAULT_PATTERN = "\\d{2}:\\d{2}:\\d{2}"
const DATE_PREFIX = "1970-01-01T"

export function parseLocalTimeColumn(column: LocalTimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  const valueExpr = format
    ? parseLocalTemporal(
        pl.pl.concatString([pl.pl.lit(DATE_PREFIX), columnExpr], ""),
        `%Y-%m-%dT${format}`,
      )
    : parseDefaultLocalTemporal(columnExpr, DEFAULT_PATTERN, DEFAULT_FORMAT, DATE_PREFIX)

  return valueExpr.cast(pl.Time).alias(column.name)
}

export function stringifyLocalTimeColumn(column: LocalTimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat ?? `${DEFAULT_FORMAT}%.f`

  return columnExpr.dt.strftime(format)
}
