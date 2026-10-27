import type { BuiltinFileDialect } from "../../models/fileDialect/builtin.ts"
import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import type { Resource } from "../../models/resource.ts"
import { getDataPath } from "../resource/data.ts"
import { inferFileDialectFormat } from "./infer.ts"
import { resolveFileDialect } from "./resolve.ts"

export async function getSupportedFileDialect<F extends BuiltinFileDialect["format"]>(
  resource: Resource,
  supportedFormats: F[],
) {
  const dataPath = getDataPath(resource)
  if (!dataPath) return undefined

  const fileDialect = (await resolveFileDialect(resource.fileDialect)) ?? {
    format: inferFileDialectFormat(resource),
  }

  return isSupportedFileDialect(fileDialect, supportedFormats) ? fileDialect : undefined
}

function isSupportedFileDialect<F extends BuiltinFileDialect["format"]>(
  fileDialect: FileDialect | { format?: string },
  supportedFormats: F[],
): fileDialect is Extract<BuiltinFileDialect, { format: F }> {
  return supportedFormats.some(format => format === fileDialect.format)
}
