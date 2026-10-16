import { loadJsonSchema } from "../../actions/jsonSchema/load.ts"
import type { DataSchema } from "../../models/dataSchema.ts"

export async function resolveDataSchema(dataSchema?: DataSchema | string) {
  if (!dataSchema) {
    return undefined
  }

  if (typeof dataSchema !== "string") {
    return dataSchema
  }

  return await loadJsonSchema(dataSchema)
}
