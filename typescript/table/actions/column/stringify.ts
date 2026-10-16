import * as pl from "nodejs-polars"
import type { ColumnMapping } from "../../models/column.ts"
import { stringifyBooleanColumn } from "./types/boolean.ts"
import { stringifyDateColumn } from "./types/date.ts"
import { stringifyDateTimeColumn } from "./types/dateTime.ts"
import { stringifyDecimalColumn } from "./types/decimal.ts"
import { stringifyIntegerColumn } from "./types/integer.ts"
import { stringifyListColumn } from "./types/list.ts"
import { stringifyNumberColumn } from "./types/number.ts"
import { stringifyTimeColumn } from "./types/time.ts"
import { stringifyUnkonwnColumn } from "./types/unknown.ts"

export function stringifyColumn(mapping: ColumnMapping, columnExpr: pl.Expr) {
  if (mapping.source.type.equals(pl.String)) {
    return columnExpr
  }

  const column = mapping.target
  switch (column.type) {
    case "boolean":
      return stringifyBooleanColumn(column, columnExpr)
    case "date":
      return stringifyDateColumn(column, columnExpr)
    case "date-time":
      return stringifyDateTimeColumn(column, columnExpr)
    case "decimal":
      return stringifyDecimalColumn(column, columnExpr)
    case "integer":
      return stringifyIntegerColumn(column, columnExpr)
    case "list":
      return stringifyListColumn(column, columnExpr)
    case "number":
      return stringifyNumberColumn(column, columnExpr)
    case "time":
      return stringifyTimeColumn(column, columnExpr)
    case "unknown":
      return stringifyUnkonwnColumn(column, columnExpr)
    default:
      return columnExpr
  }
}
