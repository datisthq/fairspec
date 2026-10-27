import { getIsFrictionlessDescriptor } from "../../actions/frictionless/convert.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import { assertTableSchema } from "./assert.ts"
import { loadTableSchema } from "./load.ts"

export async function resolveTableSchema(tableSchema?: TableSchema | string) {
  if (!tableSchema) {
    return undefined
  }

  if (typeof tableSchema !== "string") {
    const isFrictionless = getIsFrictionlessDescriptor(tableSchema, {
      kind: "tableSchema",
    })
    return isFrictionless ? await assertTableSchema(tableSchema) : tableSchema
  }

  return await loadTableSchema(tableSchema)
}
