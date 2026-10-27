import type { IntegerColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { stripNumberText } from "./-shared/text.ts"
import { NUMBER_PATTERN } from "./number.ts"

const PLAIN_INTEGER_PATTERN = "^-?(?:0|[1-9][0-9]*)$"
const MAX_INTEGER_DIGITS = 19
// Keeps the exponent arithmetic below from wrapping Int64 into a huge zero padding
const MAX_EXPONENT = 10 ** 15

export function parseIntegerColumn(column: IntegerColumn, columnExpr: pl.Expr) {
  return parseIntegerText(normalizeIntegerText(column.property, columnExpr))
}

export function stringifyIntegerColumn(_column: IntegerColumn, columnExpr: pl.Expr) {
  // Convert to string
  columnExpr = columnExpr.cast(pl.String)

  //const groupChar = column.groupChar
  //const bareNumber = column.bareNumber

  // TODO: Add group character formatting (thousands separator) when needed
  // TODO: Add non-bare number formatting (currency symbols, etc.) when needed

  return columnExpr
}

// A string categorical property can reach here, so both keywords are read untyped
export function normalizeIntegerText(
  property: { [key: string]: unknown },
  columnExpr: pl.Expr,
) {
  if (property.withText === true) {
    columnExpr = stripNumberText(columnExpr)
  }

  if (typeof property.groupChar === "string" && property.groupChar) {
    const escapedGroupChar = property.groupChar.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
    columnExpr = columnExpr.str.replaceAll(escapedGroupChar, "", false)
  }

  return columnExpr
}

/**
 * Read a JSON number whose fraction is zero as an Int64, or null outside its range
 */
export function parseIntegerText(textExpr: pl.Expr) {
  const plainExpr = pl
    .when(textExpr.str.contains(PLAIN_INTEGER_PATTERN))
    .then(textExpr)
    .otherwise(pl.lit(null))
    .cast(pl.Int64)

  // Only cells the plain form missed reach the exact reading, so the rest stay null there
  const missedExpr = pl.when(plainExpr.isNull()).then(textExpr).otherwise(pl.lit(null))
  const otherExpr = pl
    .when(missedExpr.str.contains(NUMBER_PATTERN))
    .then(missedExpr)
    .otherwise(pl.lit(null))

  const fractionExpr = otherExpr.str.extract("\\.([0-9]+)", 1).fillNull("")
  const digitsExpr = pl
    .concatString([otherExpr.str.extract("^-?([0-9]+)", 1), fractionExpr], "", false)
    .str.stripCharsStart("0")
  const trimmedExpr = digitsExpr.str.stripCharsEnd("0")
  const exponentExpr = otherExpr.str
    .extract("[eE]([+-]?[0-9]+)$", 1)
    .fillNull("0")
    .cast(pl.Int64)
    .clip(-MAX_EXPONENT, MAX_EXPONENT)
  const shiftExpr = exponentExpr
    .minus(fractionExpr.str.lengths().cast(pl.Int64))
    .plus(digitsExpr.str.lengths().cast(pl.Int64))
    .minus(trimmedExpr.str.lengths().cast(pl.Int64))
  const lengthExpr = trimmedExpr.str.lengths().cast(pl.Int64).plus(shiftExpr)

  const isWholeExpr = shiftExpr.gtEq(0).and(lengthExpr.ltEq(MAX_INTEGER_DIGITS))

  // repeatBy refuses a negative count even on rows the condition drops
  const zerosExpr = pl
    .lit("0")
    .repeatBy(pl.when(isWholeExpr).then(shiftExpr).otherwise(pl.lit(0)))
    .lst.join({ separator: "" })
  const wholeExpr = pl
    .when(trimmedExpr.eq(pl.lit("")))
    .then(pl.lit("0"))
    .when(isWholeExpr)
    .then(pl.concatString([trimmedExpr, zerosExpr], "", false))
    .otherwise(pl.lit(null))
  const signExpr = pl
    .when(otherExpr.str.startsWith("-"))
    .then(pl.lit("-"))
    .otherwise(pl.lit(""))
  const otherValueExpr = pl.concatString([signExpr, wholeExpr], "", false).cast(pl.Int64)

  return plainExpr.fillNull(otherValueExpr)
}
