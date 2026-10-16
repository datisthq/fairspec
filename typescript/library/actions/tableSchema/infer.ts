import type { Resource } from "@fairspec/metadata"
import type { InferTableSchemaOptions } from "@fairspec/table"
import { inferTableSchemaFromTable } from "@fairspec/table"
import { loadTable } from "../../actions/table/load.ts"
import { system } from "../../system.ts"

export async function inferTableSchema(
  resource: Resource,
  options?: InferTableSchemaOptions,
) {
  for (const plugin of system.plugins) {
    const schema = await plugin.inferTableSchema?.(resource, options)
    if (schema) {
      return schema
    }
  }

  const table = await loadTable(resource, { denormalized: true })
  if (!table) {
    return undefined
  }

  const schema = await inferTableSchemaFromTable(table, options)
  return schema
}
