import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { assertDataSchema } from "./assert.ts"

export async function loadDataSchema(path: string) {
  const descriptor = await loadDescriptor(path)
  const dataSchema = await assertDataSchema(descriptor)
  return dataSchema
}
