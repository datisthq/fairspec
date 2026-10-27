import { getIsFrictionlessDescriptor } from "../../actions/frictionless/convert.ts"
import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import { assertFileDialect } from "./assert.ts"
import { loadFileDialect } from "./load.ts"

export async function resolveFileDialect(fileDialect?: FileDialect | string) {
  if (!fileDialect) {
    return undefined
  }

  if (typeof fileDialect !== "string") {
    const isFrictionless = getIsFrictionlessDescriptor(fileDialect, {
      kind: "fileDialect",
    })
    return isFrictionless ? await assertFileDialect(fileDialect) : fileDialect
  }

  return await loadFileDialect(fileDialect)
}
