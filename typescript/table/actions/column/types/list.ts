import type { ListColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"

// TODO:
// Add more validation:
// - Return null instead of list if all array values are nulls?
export function parseListColumn(column: ListColumn, columnExpr: pl.Expr) {
  const delimiter = column.property.delimiter ?? ","
  const itemType = column.property.itemType

  let dtype: any = pl.String
  if (itemType === "integer") dtype = pl.Int64
  if (itemType === "number") dtype = pl.Float64
  if (itemType === "boolean") dtype = pl.Bool
  if (itemType === "date-time") dtype = pl.Datetime
  if (itemType === "date") dtype = pl.Date
  if (itemType === "time") dtype = pl.Time

  columnExpr = columnExpr.str.split(delimiter).cast(pl.List(dtype))

  return columnExpr
}

export function stringifyListColumn(column: ListColumn, columnExpr: pl.Expr) {
  const delimiter = column.property.delimiter ?? ","

  return columnExpr
    .cast(pl.List(pl.String))
    .lst.join({ separator: delimiter, ignoreNulls: true })
}
