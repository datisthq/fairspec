import { getBasePropertyType } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { ColumnMapping } from "../../models/column.ts"
import { parseBase64Column } from "./types/base64.ts"
import { parseBooleanColumn } from "./types/boolean.ts"
import { parseDateColumn } from "./types/date.ts"
import { parseDateTimeColumn } from "./types/dateTime.ts"
import { parseDecimalColumn } from "./types/decimal.ts"
import { parseEmailColumn } from "./types/email.ts"
import { parseHexColumn } from "./types/hex.ts"
import {
  normalizeIntegerText,
  parseIntegerColumn,
  parseIntegerText,
} from "./types/integer.ts"
import { parseListColumn } from "./types/list.ts"
import { parseLocalDateTimeColumn } from "./types/localDateTime.ts"
import { parseLocalTimeColumn } from "./types/localTime.ts"
import { parseNumberColumn } from "./types/number.ts"
import { parseTimeColumn } from "./types/time.ts"
import { parseUrlColumn } from "./types/url.ts"

export function parseColumn(
  mapping: ColumnMapping,
  columnExpr: pl.Expr,
  options?: { isPlain?: boolean },
) {
  if (!mapping.source.type.equals(pl.String)) {
    return columnExpr
  }

  const column = mapping.target
  switch (column.type) {
    case "base64":
      return parseBase64Column(column, columnExpr)
    case "boolean":
      return parseBooleanColumn(column, columnExpr)
    case "categorical":
      if (getBasePropertyType(column.property.type) !== "integer") return columnExpr
      return parseIntegerText(normalizeIntegerText(column.property, columnExpr))
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
      return parseIntegerColumn(column, columnExpr, options)
    case "list":
      return parseListColumn(column, columnExpr)
    case "local-date-time":
      return parseLocalDateTimeColumn(column, columnExpr)
    case "local-time":
      return parseLocalTimeColumn(column, columnExpr)
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
