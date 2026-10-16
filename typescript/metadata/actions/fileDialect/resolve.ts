import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import { loadFileDialect } from "./load.ts"

export async function resolveFileDialect(fileDialect?: FileDialect | string) {
  if (!fileDialect) {
    return undefined
  }

  if (typeof fileDialect !== "string") {
    return fileDialect
  }

  return await loadFileDialect(fileDialect)
}
