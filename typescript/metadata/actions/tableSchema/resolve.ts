import type { TableSchema } from "../../models/tableSchema.ts"
import { loadTableSchema } from "./load.ts"

export async function resolveTableSchema(tableSchema?: TableSchema | string) {
  if (!tableSchema) {
    return undefined
  }

  if (typeof tableSchema !== "string") {
    return tableSchema
  }

  return await loadTableSchema(tableSchema)
}
