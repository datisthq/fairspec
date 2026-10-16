import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import type { Resource } from "../../models/resource.ts"
import { getDataPath } from "../resource/data.ts"
import { inferFileDialectFormat } from "./infer.ts"
import { resolveFileDialect } from "./resolve.ts"

export async function getSupportedFileDialect<F extends FileDialect["format"]>(
  resource: Resource,
  supportedFormats: F[],
): Promise<Extract<FileDialect, { format: F }> | undefined> {
  const dataPath = getDataPath(resource)
  if (!dataPath) return undefined

  const fielDialect = (await resolveFileDialect(resource.fileDialect)) ?? {
    format: inferFileDialectFormat(resource),
  }

  for (const supportedFormat of supportedFormats) {
    if (fielDialect.format === supportedFormat) {
      return fielDialect as Extract<FileDialect, { format: F }>
    }
  }

  return undefined
}
