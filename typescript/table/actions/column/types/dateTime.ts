import type { DateTimeColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { keepExactTemporal } from "./-shared/temporal.ts"

const DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S"
const DEFAULT_PATTERN =
  "^(\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2})(\\.\\d+)?(Z|[+-]\\d{2}:\\d{2})?$"

// TODO: Add support for timezone handling
export function parseDateTimeColumn(column: DateTimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat
  if (format) {
    return keepExactTemporal(
      columnExpr,
      columnExpr.str.strptime(pl.Datetime, format),
      format,
    )
  }

  const textExpr = columnExpr.str.extract(DEFAULT_PATTERN, 1)
  const fractionExpr = columnExpr.str.extract(DEFAULT_PATTERN, 2).fillNull("")
  const valueExpr = pl.pl
    .concatString([textExpr, fractionExpr], "")
    .str.strptime(pl.Datetime, `${DEFAULT_FORMAT}%.f`)

  return keepExactTemporal(textExpr, valueExpr, DEFAULT_FORMAT)
}

export function stringifyDateTimeColumn(column: DateTimeColumn, columnExpr: pl.Expr) {
  const format = column.property.temporalFormat ?? DEFAULT_FORMAT

  return columnExpr.dt.strftime(format)
}
