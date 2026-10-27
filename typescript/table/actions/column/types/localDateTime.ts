import type { LocalDateTimeColumn } from "@fairspec/metadata"
import type * as pl from "nodejs-polars"
import { parseDefaultLocalTemporal, parseLocalTemporal } from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S"
const DEFAULT_PATTERN = "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}"

export function parseLocalDateTimeColumn(
  column: LocalDateTimeColumn,
  columnExpr: pl.Expr,
) {
  const format = column.property.temporalFormat
  if (format) return parseLocalTemporal(columnExpr, format)

  return parseDefaultLocalTemporal(columnExpr, DEFAULT_PATTERN, DEFAULT_FORMAT)
}

export function stringifyLocalDateTimeColumn(
  column: LocalDateTimeColumn,
  columnExpr: pl.Expr,
) {
  const format = column.property.temporalFormat ?? `${DEFAULT_FORMAT}%.f`

  return columnExpr.dt.strftime(format)
}
