import type { Column, ColumnProperty, ColumnType } from "../../models/column/column.ts"
import { getBasePropertyType, getIsNullablePropertyType } from "./property.ts"

export function createColumnFromProperty(name: string, property: ColumnProperty): Column {
  const baseType = getBasePropertyType(property.type)
  const format = "format" in property ? property.format : undefined
  const nullable = getIsNullablePropertyType(property.type) || undefined
  const columnType = getColumnType(baseType, format)

  // TODO: any way to make this more type-safe?
  return { type: columnType, name, nullable, property } as Column
}

function getColumnType(
  baseType: ReturnType<typeof getBasePropertyType>,
  format: string | undefined,
): ColumnType {
  switch (baseType) {
    case "boolean":
      return "boolean"
    case "integer":
      return format === "categorical" ? "categorical" : "integer"
    case "number":
      return "number"
    case "string":
      switch (format) {
        case "categorical":
          return "categorical"
        case "decimal":
          return "decimal"
        case "list":
          return "list"
        case "base64":
          return "base64"
        case "hex":
          return "hex"
        case "email":
          return "email"
        case "url":
          return "url"
        case "date-time":
          return "date-time"
        case "date":
          return "date"
        case "time":
          return "time"
        case "duration":
          return "duration"
        case "wkt":
          return "wkt"
        case "wkb":
          return "wkb"
        default:
          return "string"
      }
    case "array":
      return "array"
    case "object":
      switch (format) {
        case "geojson":
          return "geojson"
        case "topojson":
          return "topojson"
        default:
          return "object"
      }
    default:
      return "unknown"
  }
}
