import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import { assertFileDialect } from "./assert.ts"
import { loadFileDialect } from "./load.ts"

export async function resolveFileDialect(fileDialect?: FileDialect | string) {
  if (!fileDialect) {
    return undefined
  }

  // A dialect that is not valid never reads a file, Fairspec or Frictionless
  if (typeof fileDialect !== "string") {
    return await assertFileDialect(fileDialect)
  }

  return await loadFileDialect(fileDialect)
}
