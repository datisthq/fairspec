import * as pl from "nodejs-polars"

export function evaluateExpression(expr: pl.Expr) {
  // @ts-expect-error
  return pl.select(expr.alias("value")).toRecords()[0].value
}
