import * as pl from "nodejs-polars"

// Polars matches a format anywhere in the text (`15/01/2024` parses as May 1 under
// `%m/%d/%Y`), so a value is kept only when it prints back to its own text
export function keepExactTemporal(textExpr: pl.Expr, valueExpr: pl.Expr, format: string) {
  return pl
    .when(
      normalizeTemporalText(valueExpr.dt.strftime(format)).eq(
        normalizeTemporalText(textExpr),
      ),
    )
    .then(valueExpr)
    .otherwise(pl.lit(null))
}

/** The default patterns match only zero-padded fixed-width text, so it compares as is */
export function keepExactDefaultTemporal(
  textExpr: pl.Expr,
  valueExpr: pl.Expr,
  format: string,
) {
  // never for a custom temporalFormat: `%d` accepts `5`, which only normalising compares
  return pl
    .when(valueExpr.dt.strftime(format).eq(textExpr))
    .then(valueExpr)
    .otherwise(pl.lit(null))
}

function normalizeTemporalText(textExpr: pl.Expr) {
  return textExpr.str.toLowerCase().str.replaceAll("(^|\\D)0+(\\d)", "${1}${2}")
}
