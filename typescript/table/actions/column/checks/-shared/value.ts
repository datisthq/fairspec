import { type Column, getBasePropertyType } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { CellMapping } from "../../../../models/cell.ts"
import { normalizeIntegerText, parseIntegerText } from "../../types/integer.ts"

const NUMERIC_COLUMN_TYPES: Column["type"][] = ["integer", "number", "decimal"]

/**
 * The cell value a numeric constraint compares, or undefined for a column it ignores
 */
export function getNumericValue(column: Column, mapping: CellMapping) {
  if (NUMERIC_COLUMN_TYPES.includes(column.type)) return mapping.target

  // The target of a categorical column is its label, so the value is read again
  if (column.type === "categorical") {
    if (getBasePropertyType(column.property.type) !== "integer") return undefined
    const textExpr = mapping.source.cast(pl.String)
    return parseIntegerText(normalizeIntegerText(column.property, textExpr))
  }

  return undefined
}
