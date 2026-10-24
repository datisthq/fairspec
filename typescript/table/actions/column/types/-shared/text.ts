import * as pl from "nodejs-polars"

/**
 * Strip the text around a number: the span from the first to the last digit is kept, and a
 * minus in the stripped prefix makes it negative. Text inside the span stays, so it fails parsing
 */
export function stripNumberText(columnExpr: pl.Expr, options?: { decimalChar?: string }) {
  const decimal = options?.decimalChar ? `(?:${escapeRegex(options.decimalChar)})?` : ""
  const span = `(${decimal}[0-9](?:.*[0-9])?)[^0-9]*$`
  const negative = `^[^0-9]*-[^0-9]*?${span}`
  const positive = `^[^0-9]*?${span}`

  return pl
    .when(columnExpr.str.contains(negative))
    .then(columnExpr.str.replace(negative, "-$1"))
    .otherwise(columnExpr.str.replace(positive, "$1"))
}

function escapeRegex(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")
}
