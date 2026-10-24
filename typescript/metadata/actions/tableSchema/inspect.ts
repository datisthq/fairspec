import type { MetadataError } from "../../models/error/metadata.ts"
import type { TableSchema } from "../../models/tableSchema.ts"

/**
 * Check the key definitions of a table schema against its columns
 */
export function inspectTableSchema(
  tableSchema: TableSchema,
  options?: { rootJsonPointer?: string },
) {
  const root = options?.rootJsonPointer === "/" ? "" : (options?.rootJsonPointer ?? "")
  const names = new Set(Object.keys(tableSchema.properties ?? {}))
  const errors: MetadataError[] = []

  const checkColumns = (columns: string[], pointer: string) => {
    for (const [index, column] of columns.entries()) {
      if (names.has(column)) continue
      errors.push({
        type: "metadata",
        message: `column "${column}" is not in properties`,
        jsonPointer: `${root}${pointer}/${index}`,
      })
    }
  }

  checkColumns(tableSchema.primaryKey ?? [], "/primaryKey")
  for (const [index, uniqueKey] of (tableSchema.uniqueKeys ?? []).entries()) {
    checkColumns(uniqueKey, `/uniqueKeys/${index}`)
  }
  for (const [index, foreignKey] of (tableSchema.foreignKeys ?? []).entries()) {
    checkColumns(foreignKey.columns, `/foreignKeys/${index}/columns`)
    if (foreignKey.columns.length !== foreignKey.reference.columns.length) {
      errors.push({
        type: "metadata",
        message: "foreign key columns and reference columns differ in length",
        jsonPointer: `${root}/foreignKeys/${index}/reference/columns`,
      })
    }
  }

  return errors
}
