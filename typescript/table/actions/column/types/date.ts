import type { DateColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { keepExactTemporal } from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%Y-%m-%d"
const DEFAULT_PATTERN = "^\\d{4}-\\d{2}-\\d{2}$"

export function parseDateColumn(column: DateColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  if (format) {
    return keepExactTemporal(columnExpr, columnExpr.str.strptime(pl.Date, format), format)
  }

  return columnExpr.str
    .extract(DEFAULT_PATTERN, 0)
    .str.strptime(pl.Date, DEFAULT_FORMAT, { exact: true })
}

export function stringifyDateColumn(column: DateColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat ?? DEFAULT_FORMAT

  return columnExpr.dt.strftime(format)
}
