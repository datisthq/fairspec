import type { TimeColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import {
  formatUtcTemporal,
  parseDefaultOffsetTemporal,
  parseOffsetTemporal,
} from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%H:%M:%S"
const DEFAULT_PATTERN = "[0-9]{2}:[0-9]{2}:[0-9]{2}"
const DATE_PREFIX = "1970-01-01T"

export function parseTimeColumn(column: TimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  const valueExpr = format
    ? parseOffsetTemporal(columnExpr, format, DATE_PREFIX)
    : parseDefaultOffsetTemporal(columnExpr, DEFAULT_PATTERN, DEFAULT_FORMAT, DATE_PREFIX)

  return valueExpr.cast(pl.Time).alias(column.name)
}

export function stringifyTimeColumn(column: TimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat

  return columnExpr.dt.strftime(
    format ? formatUtcTemporal(format) : `${DEFAULT_FORMAT}%.fZ`,
  )
}
