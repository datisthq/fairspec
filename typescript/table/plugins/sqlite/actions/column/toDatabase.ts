import type { Column } from "@fairspec/metadata"
import type { SqliteColumn } from "../../models/column.ts"

export function convertColumnToDatabase(column: Column, isNullable = true): SqliteColumn {
  const databaseColumn: SqliteColumn = {
    name: column.name,
    dataType: convertType(column.type),
    isNullable,
    comment: column.property.description,
    isAutoIncrementing: false,
    hasDefaultValue: false,
  }

  return databaseColumn
}

function convertType(columnType: Column["type"]): SqliteColumn["dataType"] {
  switch (columnType) {
    case "boolean":
      return "integer"
    case "integer":
      return "integer"
    case "number":
      return "real"
    case "string":
      return "text"
    default:
      return "text"
  }
}
