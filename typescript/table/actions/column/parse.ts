import * as pl from "nodejs-polars"
import type { ColumnMapping } from "../../models/column.ts"
import { parseBase64Column } from "./types/base64.ts"
import { parseBooleanColumn } from "./types/boolean.ts"
import { parseDateColumn } from "./types/date.ts"
import { parseDateTimeColumn } from "./types/dateTime.ts"
import { parseDecimalColumn } from "./types/decimal.ts"
import { parseEmailColumn } from "./types/email.ts"
import { parseHexColumn } from "./types/hex.ts"
import { parseIntegerColumn } from "./types/integer.ts"
import { parseListColumn } from "./types/list.ts"
import { parseNumberColumn } from "./types/number.ts"
import { parseTimeColumn } from "./types/time.ts"
import { parseUrlColumn } from "./types/url.ts"

export function parseColumn(mapping: ColumnMapping, columnExpr: pl.Expr) {
  if (!mapping.source.type.equals(pl.String)) {
    return columnExpr
  }

  const column = mapping.target
  switch (column.type) {
    case "base64":
      return parseBase64Column(column, columnExpr)
    case "boolean":
      return parseBooleanColumn(column, columnExpr)
    case "date":
      return parseDateColumn(column, columnExpr)
    case "date-time":
      return parseDateTimeColumn(column, columnExpr)
    case "decimal":
      return parseDecimalColumn(column, columnExpr)
    case "email":
      return parseEmailColumn(column, columnExpr)
    case "hex":
      return parseHexColumn(column, columnExpr)
    case "integer":
      return parseIntegerColumn(column, columnExpr)
    case "list":
      return parseListColumn(column, columnExpr)
    case "number":
      return parseNumberColumn(column, columnExpr)
    case "time":
      return parseTimeColumn(column, columnExpr)
    case "url":
      return parseUrlColumn(column, columnExpr)
    default:
      return columnExpr
  }
}
