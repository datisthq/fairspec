import type { JsonSchema, Resource } from "@fairspec/metadata"
import { generateSchema } from "json-schema-it"
import { loadData } from "../../actions/data/load.ts"

export async function inferDataSchema(resource: Resource) {
  const data = await loadData(resource)
  if (!data) {
    return undefined
  }

  try {
    return generateSchema(data) as JsonSchema
  } catch {
    return undefined
  }
}
