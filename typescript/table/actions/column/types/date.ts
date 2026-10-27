import type { DateColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { keepExactTemporal, parseLocalTemporal } from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%Y-%m-%d"
const DEFAULT_PATTERN = "^\\d{4}-\\d{2}-\\d{2}$"
const TIME_SPECIFIERS = ["%H", "%I", "%M", "%S", "%p", "%.f"]

export function parseDateColumn(column: DateColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  if (format) {
    // A Date cannot print time fields, so their round trip goes through a Datetime
    if (TIME_SPECIFIERS.some(specifier => format.includes(specifier))) {
      return parseLocalTemporal(columnExpr, format).cast(pl.Date)
    }

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
