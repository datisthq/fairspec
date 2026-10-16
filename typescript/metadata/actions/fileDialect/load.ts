import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { assertFileDialect } from "./assert.ts"

export async function loadFileDialect(path: string) {
  const descriptor = await loadDescriptor(path)
  const fileDialect = await assertFileDialect(descriptor)
  return fileDialect
}
