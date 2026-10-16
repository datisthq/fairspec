import type { Column, TableSchema } from "@fairspec/metadata"
import { getColumns } from "@fairspec/metadata"
import type { CkanField, CkanFieldInfo } from "../../models/field.ts"
import type { CkanSchema } from "../../models/schema.ts"

export function convertTableSchemaToCkan(tableSchema: TableSchema): CkanSchema {
  const fields: CkanField[] = []

  const columns = getColumns(tableSchema)
  for (const column of columns) {
    fields.push(convertColumn(column))
  }

  return { fields }
}

function convertColumn(column: Column): CkanField {
  const { title, description } = column.property

  const ckanField: CkanField = {
    id: column.name,
    type: convertType(column),
  }

  if (title || description) {
    const columnInfo: CkanFieldInfo = {} as CkanFieldInfo

    if (title) columnInfo.label = title
    if (description) columnInfo.notes = description

    columnInfo.type_override = convertType(column)

    ckanField.info = columnInfo
  }

  return ckanField
}

function convertType(column: Column): string {
  switch (column.type) {
    case "string":
      return "text"
    case "integer":
      return "int"
    case "number":
      return "numeric"
    case "boolean":
      return "bool"
    case "date":
      return "date"
    case "time":
      return "time"
    case "date-time":
      return "timestamp"
    case "object":
      return "json"
    case "array":
      return "array"
    default:
      return "text"
  }
}
