import type { DateTimeColumn } from "@fairspec/metadata"
import type * as pl from "nodejs-polars"
import {
  formatUtcTemporal,
  parseDefaultOffsetTemporal,
  parseOffsetTemporal,
} from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S"
const DEFAULT_PATTERN = "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}"

export function parseDateTimeColumn(column: DateTimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  if (format) return parseOffsetTemporal(columnExpr, format)

  return parseDefaultOffsetTemporal(columnExpr, DEFAULT_PATTERN, DEFAULT_FORMAT)
}

export function stringifyDateTimeColumn(column: DateTimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  const utcExpr = columnExpr.dt.convertTimeZone("UTC")
  if (format) return utcExpr.dt.strftime(formatUtcTemporal(format))

  return utcExpr.dt.strftime(`${DEFAULT_FORMAT}%.fZ`)
}
