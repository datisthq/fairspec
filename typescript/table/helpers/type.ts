import type * as pl from "nodejs-polars"

/** Returns the time zone of a Polars Datetime type, or undefined for a naive one */
export function getTypeTimeZone(type: pl.DataType) {
  if (type.variant !== "Datetime" || !("timeZone" in type)) return undefined
  return typeof type.timeZone === "string" && type.timeZone ? type.timeZone : undefined
}
