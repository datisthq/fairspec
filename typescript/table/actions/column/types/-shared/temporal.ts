import * as pl from "nodejs-polars"

const CANONICAL_FORMAT = "%Y-%m-%dT%H:%M:%S%.f"
const OFFSET_PATTERN = "Z|[+-][0-9]{2}:[0-9]{2}"
const LEAP_SECOND_PATTERN = "([0-9]{2}:[0-9]{2}:)60"
const OFFSET_SPECIFIERS = ["%z", "%:z"]

/**
 * Parses date-times written with a UTC offset into UTC: the text without its offset
 * goes through the exact parse of a local value, then the offset is applied
 */
export function parseOffsetTemporal(columnExpr: pl.Expr, format: string, prefix = "") {
  const tokens = splitTemporalFormat(format)
  const index = tokens.findIndex(token => OFFSET_SPECIFIERS.includes(token))
  if (index === -1) {
    return parseLocalTemporal(
      pl.pl.concatString([pl.pl.lit(prefix), columnExpr], ""),
      `${createPrefixFormat(prefix)}${format}`,
    ).dt.replaceTimeZone("UTC")
  }

  const before = tokens.slice(0, index)
  const after = tokens.slice(index + 1)
  const pattern = `(?i)^(${createTokensPattern(before)})([+-]\\d{2}:?\\d{2})(${createTokensPattern(after)})$`

  const textExpr = pl.pl.concatString(
    [
      pl.pl.lit(prefix),
      columnExpr.str.extract(pattern, 1),
      columnExpr.str.extract(pattern, 3),
    ],
    "",
  )
  const localFormat = `${createPrefixFormat(prefix)}${before.join("")}${after.join("")}`
  const localExpr = parseLocalTemporal(textExpr, localFormat)

  return applyOffset(localExpr, columnExpr.str.extract(pattern, 2))
}

/**
 * Parses an RFC 3339 date-time or time with fractional seconds and a `Z` or `±HH:MM`
 * offset, `T` and `Z` in any case
 */
export function parseDefaultOffsetTemporal(
  columnExpr: pl.Expr,
  textPattern: string,
  format: string,
  prefix = "",
) {
  const textExpr = columnExpr.str.toUpperCase()
  const pattern = `^(${textPattern})(\\.[0-9]+)?(${OFFSET_PATTERN})$`
  const localExpr = parseDefaultTemporal(textExpr, pattern, format, prefix)

  return keepLeapSecond(
    textExpr,
    applyOffset(localExpr, textExpr.str.extract(pattern, 3)),
  )
}

/** Parses an RFC 3339 date-time or time with fractional seconds and no offset */
export function parseDefaultLocalTemporal(
  columnExpr: pl.Expr,
  textPattern: string,
  format: string,
  prefix = "",
) {
  const textExpr = columnExpr.str.toUpperCase()
  const pattern = `^(${textPattern})(\\.[0-9]+)?$`

  return keepLeapSecond(textExpr, parseDefaultTemporal(textExpr, pattern, format, prefix))
}

/** Parses a value written in a custom `temporalFormat` without an offset */
export function parseLocalTemporal(textExpr: pl.Expr, format: string) {
  return keepExactTemporal(textExpr, textExpr.str.strptime(pl.Datetime, format), format)
}

/** Writes a UTC value in a pattern, the offset specifications always being zero */
export function formatUtcTemporal(format: string) {
  return splitTemporalFormat(format)
    .map(token => (token === "%z" ? "+0000" : token === "%:z" ? "+00:00" : token))
    .join("")
}

// Polars matches a format anywhere in the text (`15/01/2024` parses as May 1 under
// `%m/%d/%Y`), so a value is kept only when it prints back to its own text
export function keepExactTemporal(textExpr: pl.Expr, valueExpr: pl.Expr, format: string) {
  const tokens = splitTemporalFormat(format)

  return pl
    .when(
      normalizeTemporalText(valueExpr.dt.strftime(format), tokens).eq(
        normalizeTemporalText(textExpr, tokens),
      ),
    )
    .then(valueExpr)
    .otherwise(pl.lit(null))
}

function parseDefaultTemporal(
  columnExpr: pl.Expr,
  pattern: string,
  format: string,
  prefix: string,
) {
  const textExpr = pl.pl.concatString(
    [
      pl.pl.lit(prefix),
      columnExpr.str.extract(pattern, 1).str.replace(LEAP_SECOND_PATTERN, "${1}59"),
    ],
    "",
  )
  const fractionExpr = columnExpr.str.extract(pattern, 2).fillNull("")
  const fullFormat = `${createPrefixFormat(prefix)}${format}`

  // The pattern already fixes a zero-padded shape; exact rejects what matches only in
  // part, such as 2100-02-29, which a loose parse reads as year 0000
  return pl.pl
    .concatString([textExpr, fractionExpr], "")
    .str.strptime(pl.Datetime, `${fullFormat}%.f`, { exact: true })
}

// A second `60` reads as 23:59:59, and only where the value is 23:59 in UTC
function keepLeapSecond(textExpr: pl.Expr, valueExpr: pl.Expr) {
  return pl
    .when(
      textExpr.str
        .contains(LEAP_SECOND_PATTERN)
        .not()
        .or(valueExpr.dt.hour().eq(23).and(valueExpr.dt.minute().eq(59))),
    )
    .then(valueExpr)
    .otherwise(pl.lit(null))
}

function applyOffset(localExpr: pl.Expr, offsetExpr: pl.Expr) {
  const normalizedOffsetExpr = offsetExpr.str
    .replace("^[Zz]$", "+00:00")
    .str.replace("^([+-][0-9]{2}):?([0-9]{2})$", "${1}:${2}")

  return pl.pl
    .concatString([localExpr.dt.strftime(CANONICAL_FORMAT), normalizedOffsetExpr], "")
    .str.strptime(pl.Datetime, `${CANONICAL_FORMAT}%:z`)
}

function splitTemporalFormat(format: string) {
  return Array.from(format.matchAll(/%[.:]?.|[^%]/gs), match => match[0])
}

function createTokensPattern(tokens: string[]) {
  return tokens
    .map(token => {
      if (token === "%.f") return "(?:\\.\\d+)?"
      if (token === "%%") return "%"
      if (["%b", "%B", "%p"].includes(token)) return "[a-z]+"
      if (token.startsWith("%")) return "\\d+"
      return token.replace(/[.*+?^${}()|[\]\\-]/g, "\\$&")
    })
    .join("")
}

function createPrefixFormat(prefix: string) {
  return prefix ? "%Y-%m-%dT" : ""
}

function normalizeTemporalText(textExpr: pl.Expr, tokens: string[]) {
  let normalizedExpr = textExpr.str
    .toLowerCase()
    .str.replaceAll("(^|\\D)0+(\\d)", "${1}${2}")

  // `%B` reads `Jan` too but prints `January`, so names compare by their first letters
  if (tokens.includes("%B")) {
    normalizedExpr = normalizedExpr.str.replaceAll("([a-z]{3})[a-z]+", "${1}")
  }

  // `%.f` reads `.5` but prints `.500`, so fractions compare without trailing zeros
  if (tokens.includes("%.f")) {
    normalizedExpr = normalizedExpr.str
      .replaceAll("\\.(\\d*?)0*(\\D|$)", ".${1}${2}")
      .str.replaceAll("\\.(\\D|$)", "${1}")
  }

  return normalizedExpr
}
