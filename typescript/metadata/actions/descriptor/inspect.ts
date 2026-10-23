import type { z } from "zod"
import type { MetadataError } from "../../models/error/metadata.ts"

/**
 * Parse a descriptor with a built-in model, reporting each mismatch as a metadata error
 */
export function inspectModel<T extends z.ZodType>(
  descriptor: unknown,
  options: {
    model: T
    rootJsonPointer?: string
  },
) {
  const result = options.model.safeParse(descriptor)
  if (result.success) {
    return { data: result.data, errors: [] }
  }

  const root = options.rootJsonPointer === "/" ? "" : (options.rootJsonPointer ?? "")
  const errors: MetadataError[] = result.error.issues.map(issue => ({
    type: "metadata",
    message: issue.message,
    jsonPointer: root + issue.path.map(key => `/${escapePointerKey(key)}`).join(""),
  }))

  return { data: undefined, errors }
}

function escapePointerKey(key: PropertyKey) {
  return String(key).replaceAll("~", "~0").replaceAll("/", "~1")
}
