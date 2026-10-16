import type { Column } from "@fairspec/metadata"
import { createColumnFromProperty } from "@fairspec/metadata"
import type { SqliteColumn } from "../../models/column.ts"

export function convertColumnFromDatabase(databaseColumn: SqliteColumn): Column {
  const property = convertProperty(databaseColumn.dataType)
  const name = databaseColumn.name
  const column = createColumnFromProperty(name, property)

  if (databaseColumn.comment) {
    column.property.description = databaseColumn.comment
  }

  return column
}

function convertProperty(databaseType: SqliteColumn["dataType"]): Column["property"] {
  switch (databaseType.toLowerCase()) {
    case "blob":
      return { type: "string" }
    case "text":
      return { type: "string" }
    case "integer":
      return { type: "integer" }
    case "numeric":
    case "real":
      return { type: "number" }
    default:
      return {}
  }
}
