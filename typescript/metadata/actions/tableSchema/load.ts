import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { assertTableSchema } from "./assert.ts"

export async function loadTableSchema(path: string) {
  const descriptor = await loadDescriptor(path)
  const tableSchema = await assertTableSchema(descriptor)
  return tableSchema
}
