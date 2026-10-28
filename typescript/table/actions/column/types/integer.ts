import type { IntegerColumn } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { createFieldExpr } from "../../../helpers/expr.ts"
import { stripNumberText } from "./-shared/text.ts"

const PLAIN_INTEGER_PATTERN = "^-?(?:0|[1-9][0-9]*)$"
// NUMBER_PATTERN (number.ts) with its sign, integer, fraction and exponent captured
const NUMBER_PARTS_PATTERN = "^(-?)(0|[1-9][0-9]*)(?:\\.([0-9]+))?(?:[eE]([+-]?[0-9]+))?$"
const MAX_INTEGER_DIGITS = 19
// Keeps the exponent arithmetic below from wrapping Int64
const MAX_EXPONENT = 10 ** 15

export function parseIntegerColumn(
  column: IntegerColumn,
  columnExpr: pl.Expr,
  options?: { isPlain?: boolean },
) {
  const textExpr = normalizeIntegerText(column.property, columnExpr)
  return options?.isPlain ? parsePlainIntegerText(textExpr) : parseIntegerText(textExpr)
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
  const plainExpr = parsePlainIntegerText(textExpr)
  const field = createFieldExpr

  // Only a cell with a point or an exponent can be a whole number the plain form misses,
  // so the rest stay null there. The value is its significant digits (`core`) and
  // `shift` zeros; each step is a struct field, so no step is evaluated twice
  const otherValueExpr = pl
    .struct([
      pl
        .when(textExpr.str.contains("[.eE]"))
        .then(textExpr)
        .otherwise(pl.lit(null))
        .alias("text"),
    ])
    .struct.withFields(
      ["sign", "integer", "fraction", "exponent"].map((name, index) =>
        field("text")
          .str.extract(NUMBER_PARTS_PATTERN, index + 1)
          .alias(name),
      ),
    )
    .struct.withFields([
      pl
        .concatString([field("integer"), field("fraction").fillNull("")], "", false)
        .str.stripCharsStart("0")
        .alias("significant"),
      field("fraction")
        .str.lengths()
        .cast(pl.Int64)
        .fillNull(pl.lit(0).cast(pl.Int64))
        .alias("fractionLength"),
      pl
        .when(field("exponent").isNull())
        .then(pl.lit(0).cast(pl.Int64))
        .otherwise(field("exponent").cast(pl.Int64).clip(-MAX_EXPONENT, MAX_EXPONENT))
        .alias("exponent"),
    ])
    .struct.withFields([field("significant").str.stripCharsEnd("0").alias("core")])
    .struct.withFields([
      field("core").str.lengths().cast(pl.Int64).alias("coreLength"),
      field("exponent")
        .minus(field("fractionLength"))
        .plus(field("significant").str.lengths().cast(pl.Int64))
        .minus(field("core").str.lengths().cast(pl.Int64))
        .alias("shift"),
    ])
    .struct.withFields([
      field("shift")
        .gtEq(0)
        .and(field("coreLength").plus(field("shift")).ltEq(MAX_INTEGER_DIGITS))
        .alias("isWhole"),
    ])
    .struct.withFields([
      pl
        .when(field("coreLength").eq(0))
        .then(pl.lit("0"))
        .when(field("isWhole"))
        .then(
          pl.concatString(
            [
              field("core"),
              // repeatBy refuses a negative count even on rows the condition drops
              pl
                .lit("0")
                .repeatBy(
                  pl.when(field("isWhole")).then(field("shift")).otherwise(pl.lit(0)),
                )
                .lst.join({ separator: "" }),
            ],
            "",
            false,
          ),
        )
        .otherwise(pl.lit(null))
        .alias("whole"),
    ])
    .struct.withFields([
      pl
        .concatString(
          [
            pl
              .when(field("sign").eq(pl.lit("-")))
              .then(pl.lit("-"))
              .otherwise(pl.lit("")),
            field("whole"),
          ],
          "",
          false,
        )
        .cast(pl.Int64)
        .alias("value"),
    ])
    .struct.field("value")

  return plainExpr.fillNull(otherValueExpr)
}

/** Read only an integer written without a fraction or an exponent as an Int64 */
export function parsePlainIntegerText(textExpr: pl.Expr) {
  return pl
    .when(textExpr.str.contains(PLAIN_INTEGER_PATTERN))
    .then(textExpr)
    .otherwise(pl.lit(null))
    .cast(pl.Int64)
}
