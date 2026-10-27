import type { CellMultipleOfError, Column } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { CellMapping } from "../../../models/cell.ts"
import { normalizeNumberText } from "../types/decimal.ts"

const MAX_DECIMAL_DIGITS = 38
const INTEGER_DECIMAL = pl.Decimal(MAX_DECIMAL_DIGITS, 0)
const MAX_VALUE_DIGITS = MAX_DECIMAL_DIGITS - 1

export function checkCellMultipleOf(column: Column, mapping: CellMapping) {
  const property = column.property
  if (!("multipleOf" in property)) return undefined

  const multipleOf = property.multipleOf
  if (typeof multipleOf !== "number") return undefined

  const errorTemplate: CellMultipleOfError = {
    type: "cell/multipleOf",
    columnName: column.name,
    multipleOf,
    rowNumber: 0,
    cell: "",
  }

  const isFloatErrorExpr = mapping.target.cast(pl.Float64).modulo(multipleOf).eq(0).not()

  const [integerDigits = "", fractionDigits = ""] =
    formatDecimalText(multipleOf).split(".")
  const scale = fractionDigits.length
  const divisor = `${integerDigits}${fractionDigits}`.replace(/^0+/, "")
  if (!divisor || divisor.length > MAX_DECIMAL_DIGITS || scale >= MAX_VALUE_DIGITS) {
    return { isErrorExpr: isFloatErrorExpr, errorTemplate }
  }

  // The value scaled by 10^scale must be an integer of at most 37 digits: the quotient rounds,
  // so multiplied back it can pass 38 and abort the query; anything else is checked as a float
  const textExpr = selectValueText(column, mapping)
  const isExactExpr = textExpr.str
    .contains(`^[+-]?0*[0-9]{0,${MAX_VALUE_DIGITS - scale}}(?:\\.[0-9]*)?$`)
    .and(textExpr.str.contains("[0-9]"))

  const hasExcessDigitsExpr = textExpr.str.contains(`\\.[0-9]{${scale}}[0-9]*[1-9]`)
  const integerExpr = textExpr.str.replace("\\..*$", "").str.replace("^([+-]?)", "${1}0")
  const fractionExpr = pl
    .concatString(
      [textExpr.str.extract("\\.([0-9]*)", 1).fillNull(""), pl.lit("0".repeat(scale))],
      "",
    )
    .str.slice(0, scale)

  const scaledExpr = pl
    .when(isExactExpr)
    .then(pl.concatString([integerExpr, fractionExpr], ""))
    .otherwise(pl.lit("0"))
    .cast(INTEGER_DECIMAL)

  const divisorExpr = pl.lit(divisor).cast(INTEGER_DECIMAL)
  const isExactErrorExpr = hasExcessDigitsExpr.or(
    scaledExpr.divideBy(divisorExpr).mul(divisorExpr).neq(scaledExpr),
  )

  const isErrorExpr = pl
    .when(isExactExpr)
    .then(isExactErrorExpr)
    .otherwise(isFloatErrorExpr)

  return { isErrorExpr, errorTemplate }
}

// A float target keeps no digit past double precision, so the text as written is used
function selectValueText(column: Column, mapping: CellMapping) {
  const targetTextExpr = mapping.target.cast(pl.String)
  if (column.type !== "number" && column.type !== "decimal") return targetTextExpr

  const sourceTextExpr = normalizeNumberText(column, mapping.source.cast(pl.String))
  return pl
    .when(sourceTextExpr.cast(pl.Float64).eq(mapping.target.cast(pl.Float64)))
    .then(sourceTextExpr)
    .otherwise(targetTextExpr)
}

function formatDecimalText(value: number) {
  const text = String(value)
  const [mantissa = "", exponentText] = text.split("e")
  if (exponentText === undefined) return text

  const [integer = "", fraction = ""] = mantissa.split(".")
  const digits = `${integer}${fraction}`
  const point = integer.length + Number(exponentText)
  if (point <= 0) return `0.${"0".repeat(-point)}${digits}`
  if (point >= digits.length) return `${digits}${"0".repeat(point - digits.length)}`
  return `${digits.slice(0, point)}.${digits.slice(point)}`
}
