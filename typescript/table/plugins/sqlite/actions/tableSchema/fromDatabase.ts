import type { Column } from "@fairspec/metadata"
import { getColumnProperties } from "@fairspec/metadata"
import type { SqliteSchema } from "../../models/schema.ts"
import { convertColumnFromDatabase } from "../column/fromDatabase.ts"

export function convertTableSchemaFromDatabase(databaseSchema: SqliteSchema) {
  const columns: Column[] = []
  const required: string[] = []

  for (const databaseColumn of databaseSchema.columns) {
    const column = convertColumnFromDatabase(databaseColumn)

    if (databaseColumn.isNullable) {
      const baseType = column.property.type
      if (baseType && typeof baseType === "string") {
        ;(column.property as Record<string, unknown>).type = [baseType, "null"]
      }
    }

    columns.push(column)

    if (!databaseColumn.isNullable) {
      required.push(databaseColumn.name)
    }
  }

  return {
    properties: getColumnProperties(columns),
    primaryKey: databaseSchema.primaryKey,
    required,
  }
}
