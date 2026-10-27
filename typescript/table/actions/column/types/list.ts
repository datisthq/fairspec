import type { ListColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import {
  parseDefaultLocalTemporal,
  parseDefaultOffsetTemporal,
} from "./-shared/temporal.ts"
import { parseIntegerText } from "./integer.ts"
import { parseNumberText } from "./number.ts"

const TRUE_VALUES = ["true", "True", "TRUE", "1"]
const FALSE_VALUES = ["false", "False", "FALSE", "0"]
const DATE_TIME_PATTERN = "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}"
const DATE_TIME_FORMAT = "%Y-%m-%dT%H:%M:%S"
const TIME_PATTERN = "[0-9]{2}:[0-9]{2}:[0-9]{2}"
const TIME_FORMAT = "%H:%M:%S"
const DATE_PREFIX = "1970-01-01T"
const DATE_PATTERN = "^[0-9]{4}-[0-9]{2}-[0-9]{2}$"

export function parseListColumn(column: ListColumn, columnExpr: pl.Expr) {
  const delimiter = column.property.delimiter ?? ","
  const itemType = column.property.itemType

  // An empty cell is an empty list, while every delimiter splits off an item, empty or not
  const splitExpr = columnExpr.str.split(delimiter)
  const itemsExpr = pl
    .when(columnExpr.eq(pl.lit("")))
    .then(splitExpr.lst.head(0))
    .otherwise(splitExpr)
  if (!itemType || itemType === "string") return itemsExpr

  const listExpr = itemsExpr.lst.eval(castItem(itemType, pl.element()))

  // An unparsable item becomes null rather than failing, so a null item marks the cell
  return pl
    .when(countItems(listExpr, pl.element().isNull()).gt(0))
    .then(pl.lit(null))
    .otherwise(listExpr)
}

export function stringifyListColumn(column: ListColumn, columnExpr: pl.Expr) {
  const delimiter = column.property.delimiter ?? ","

  return columnExpr
    .cast(pl.List(pl.String))
    .lst.join({ separator: delimiter, ignoreNulls: true })
}

function castItem(itemType: ListColumn["property"]["itemType"], itemExpr: pl.Expr) {
  switch (itemType) {
    case "integer":
      return parseIntegerText(itemExpr)
    case "number":
      return parseNumberText(itemExpr)
    case "boolean":
      return pl
        .when(itemExpr.isIn(TRUE_VALUES))
        .then(pl.lit(true))
        .when(itemExpr.isIn(FALSE_VALUES))
        .then(pl.lit(false))
        .otherwise(pl.lit(null).cast(pl.Bool))
    case "date-time":
      return parseDefaultOffsetTemporal(itemExpr, DATE_TIME_PATTERN, DATE_TIME_FORMAT)
    case "local-date-time":
      return parseDefaultLocalTemporal(itemExpr, DATE_TIME_PATTERN, DATE_TIME_FORMAT)
    case "date":
      return itemExpr.str
        .extract(DATE_PATTERN, 0)
        .str.strptime(pl.Date, "%Y-%m-%d", { exact: true })
    case "time":
      return parseDefaultOffsetTemporal(
        itemExpr,
        TIME_PATTERN,
        TIME_FORMAT,
        DATE_PREFIX,
      ).cast(pl.Time)
    case "local-time":
      return parseDefaultLocalTemporal(
        itemExpr,
        TIME_PATTERN,
        TIME_FORMAT,
        DATE_PREFIX,
      ).cast(pl.Time)
    default:
      return itemExpr
  }
}

function countItems(listExpr: pl.Expr, predicateExpr: pl.Expr) {
  return listExpr.lst.eval(predicateExpr.cast(pl.UInt32)).lst.sum()
}
