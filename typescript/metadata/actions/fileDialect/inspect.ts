import type { MetadataError } from "../../models/error/metadata.ts"
import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"

/**
 * Checks what the profile cannot: a row listed both in `headerRows` and `commentRows`
 */
export function inspectFileDialect(
  fileDialect: FileDialect,
  options?: { rootJsonPointer?: string },
) {
  const root = options?.rootJsonPointer === "/" ? "" : (options?.rootJsonPointer ?? "")
  const errors: MetadataError[] = []

  if (!("commentRows" in fileDialect) || !Array.isArray(fileDialect.commentRows)) {
    return errors
  }
  if ("rowType" in fileDialect && fileDialect.rowType === "object") return errors

  const declared = "headerRows" in fileDialect ? fileDialect.headerRows : undefined
  const headerRows: unknown[] = Array.isArray(declared)
    ? declared
    : declared === false
      ? []
      : [1]
  const rows = fileDialect.commentRows.filter(row => headerRows.includes(row))
  if (rows.length) {
    errors.push({
      type: "metadata",
      message: `row ${rows.join(", ")} is listed both in headerRows and commentRows`,
      jsonPointer: `${root}/commentRows`,
    })
  }

  return errors
}
