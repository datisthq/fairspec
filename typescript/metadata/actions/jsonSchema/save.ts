import { saveDescriptor } from "../../actions/descriptor/save.ts"
import type { JsonSchema } from "../../models/jsonSchema.ts"

export async function saveJsonSchema(
  jsonSchema: JsonSchema,
  options: {
    path: string
    overwrite?: boolean
  },
) {
  await saveDescriptor(jsonSchema, {
    path: options.path,
    overwrite: options.overwrite,
  })
}
