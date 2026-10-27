import type { MetadataError } from "../../models/error/metadata.ts"
import type { ForeignKey } from "../../models/foreignKey.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import { createColumnFromProperty } from "../column/create.ts"

const NUMERIC_COLUMN_TYPES = ["integer", "number", "decimal"]

/**
 * Check that each local column of a foreign key has the column type of the column it refers to
 */
export function inspectForeignKeyTypes(
  foreignKey: ForeignKey,
  options: {
    tableSchema: TableSchema
    referencedSchema: TableSchema
    jsonPointer: string
  },
) {
  const errors: MetadataError[] = []

  for (const [index, name] of foreignKey.columns.entries()) {
    const referencedName = foreignKey.reference.columns[index]
    const type = getColumnType(options.tableSchema, name)
    const referencedType = getColumnType(options.referencedSchema, referencedName)
    if (!type || !referencedType || type === referencedType) continue
    if (
      NUMERIC_COLUMN_TYPES.includes(type) &&
      NUMERIC_COLUMN_TYPES.includes(referencedType)
    ) {
      continue
    }

    errors.push({
      type: "metadata",
      message: `foreign key column "${name}" is ${type}, but "${referencedName}" is ${referencedType}`,
      jsonPointer: options.jsonPointer,
    })
    break
  }

  return errors
}

function getColumnType(tableSchema: TableSchema, name?: string) {
  const properties = tableSchema.properties ?? {}
  if (name === undefined || !Object.hasOwn(properties, name)) return undefined
  const property = properties[name]
  return property ? createColumnFromProperty(name, property).type : undefined
}
