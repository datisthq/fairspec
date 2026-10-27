import type { Descriptor } from "../../models/descriptor.ts"
import type { MetadataError } from "../../models/error/metadata.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import { isValidEcmaRegex } from "../../actions/jsonSchema/regex.ts"

const NULLABLE_KEYWORDS = ["enum", "const", "default", "examples"]

/**
 * Check the required columns, patterns and key definitions of a table schema against its columns
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

  checkColumns(tableSchema.required ?? [], "/required")
  for (const [name, property] of Object.entries(tableSchema.properties ?? {})) {
    if (!("pattern" in property) || typeof property.pattern !== "string") continue
    if (!isValidEcmaRegex(property.pattern)) {
      errors.push({
        type: "metadata",
        message: `pattern "${property.pattern}" is not a valid regular expression`,
        jsonPointer: `${root}/properties/${escapeJsonPointer(name)}/pattern`,
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

/**
 * Check that enum, const, default and examples hold null only where the column type allows it
 */
export function inspectTableSchemaNulls(
  descriptor: Descriptor,
  options?: { rootJsonPointer?: string },
) {
  const root = options?.rootJsonPointer === "/" ? "" : (options?.rootJsonPointer ?? "")
  const errors: MetadataError[] = []
  const properties = isDescriptor(descriptor.properties) ? descriptor.properties : {}

  for (const [name, property] of Object.entries(properties)) {
    if (!isDescriptor(property) || property.type === undefined) continue
    if (Array.isArray(property.type) && property.type.includes("null")) continue

    const pointer = `${root}/properties/${escapeJsonPointer(name)}`
    const message = `null is not allowed: the type of column "${name}" does not include "null"`
    for (const keyword of NULLABLE_KEYWORDS) {
      const value = property[keyword]
      if (keyword === "enum" || keyword === "examples") {
        if (!Array.isArray(value)) continue
        for (const [index, item] of value.entries()) {
          if (item !== null) continue
          errors.push({
            type: "metadata",
            message,
            jsonPointer: `${pointer}/${keyword}/${index}`,
          })
        }
      } else if (value === null) {
        errors.push({ type: "metadata", message, jsonPointer: `${pointer}/${keyword}` })
      }
    }
  }

  return errors
}

function isDescriptor(value: unknown): value is Descriptor {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}

function escapeJsonPointer(key: string) {
  return key.replaceAll("~", "~0").replaceAll("/", "~1")
}
