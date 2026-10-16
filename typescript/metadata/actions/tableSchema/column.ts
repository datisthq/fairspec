import { objectEntries } from "ts-extras"
import { createColumnFromProperty } from "../../actions/column/create.ts"
import type { Column } from "../../models/column/column.ts"
import type { TableSchema } from "../../models/tableSchema.ts"

export function getColumns(tableSchema: TableSchema) {
  const columns: Column[] = []

  for (const [name, property] of objectEntries(tableSchema.properties ?? {})) {
    const column = createColumnFromProperty(name, property)
    column.required = tableSchema.allRequired || tableSchema.required?.includes(name)
    columns.push(column)
  }

  return columns
}
