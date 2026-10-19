import type { ListColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

const TRUE_VALUES = ["true", "True", "TRUE", "1"]
const FALSE_VALUES = ["false", "False", "FALSE", "0"]

export function parseListColumn(column: ListColumn, columnExpr: pl.Expr) {
  const delimiter = column.property.delimiter ?? ","

  const itemsExpr = columnExpr.str.split(delimiter)
  const listExpr = itemsExpr.lst.eval(parseItem(column, pl.element()))

  // An unparsable item becomes null rather than failing, and an empty one is null
  // by design, so a list holding more nulls than empty items had an invalid item
  return pl
    .when(
      countItems(listExpr, pl.element().isNull()).gt(
        countItems(itemsExpr, pl.element().eq(pl.lit(""))),
      ),
    )
    .then(pl.lit(null))
    .otherwise(listExpr)
}

export function stringifyListColumn(column: ListColumn, columnExpr: pl.Expr) {
  const delimiter = column.property.delimiter ?? ","

  return columnExpr
    .cast(pl.List(pl.String))
    .lst.join({ separator: delimiter, ignoreNulls: true })
}

function parseItem(column: ListColumn, itemExpr: pl.Expr) {
  const itemType = column.property.itemType
  if (!itemType || itemType === "string") return itemExpr

  return pl
    .when(itemExpr.eq(pl.lit("")))
    .then(pl.lit(null))
    .otherwise(castItem(itemType, itemExpr))
}

function castItem(itemType: ListColumn["property"]["itemType"], itemExpr: pl.Expr) {
  switch (itemType) {
    case "integer":
      return itemExpr.cast(pl.Int64)
    case "number":
      return itemExpr.cast(pl.Float64)
    case "boolean":
      return pl
        .when(itemExpr.isIn(TRUE_VALUES))
        .then(pl.lit(true))
        .when(itemExpr.isIn(FALSE_VALUES))
        .then(pl.lit(false))
        .otherwise(pl.lit(null).cast(pl.Bool))
    case "date-time":
      return itemExpr.cast(pl.Datetime("us"))
    case "date":
      return itemExpr.cast(pl.Date)
    case "time":
      return itemExpr.cast(pl.Time)
    default:
      return itemExpr
  }
}

function countItems(listExpr: pl.Expr, predicateExpr: pl.Expr) {
  return listExpr.lst.eval(predicateExpr.cast(pl.UInt32)).lst.sum()
}
