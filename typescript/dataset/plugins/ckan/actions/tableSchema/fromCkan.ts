import type { Column, TableSchema } from "@fairspec/metadata"
import { getColumnProperties } from "@fairspec/metadata"
import type { CkanField } from "../../models/field.ts"
import type { CkanSchema } from "../../models/schema.ts"

export function convertTableSchemaFromCkan(ckanSchema: CkanSchema): TableSchema {
  const columns: Column[] = []

  for (const ckanField of ckanSchema.fields) {
    columns.push(convertColumn(ckanField))
  }

  return { properties: getColumnProperties(columns) }
}

function convertColumn(ckanField: CkanField): Column {
  const { info } = ckanField

  const baseProperty: {
    title?: string
    description?: string
  } = {}

  if (info) {
    if (info.label) baseProperty.title = info.label
    if (info.notes) baseProperty.description = info.notes
  }

  const columnType = (info?.type_override || ckanField.type).toLowerCase()
  switch (columnType) {
    case "text":
    case "string":
      return {
        name: ckanField.id,
        type: "string",
        property: { ...baseProperty, type: "string" },
      }
    case "int":
    case "integer":
      return {
        name: ckanField.id,
        type: "integer",
        property: { ...baseProperty, type: "integer" },
      }
    case "numeric":
    case "number":
    case "float":
      return {
        name: ckanField.id,
        type: "number",
        property: { ...baseProperty, type: "number" },
      }
    case "bool":
    case "boolean":
      return {
        name: ckanField.id,
        type: "boolean",
        property: { ...baseProperty, type: "boolean" },
      }
    case "date":
      return {
        name: ckanField.id,
        type: "date",
        property: { ...baseProperty, type: "string", format: "date" },
      }
    case "time":
      return {
        name: ckanField.id,
        type: "time",
        property: { ...baseProperty, type: "string", format: "time" },
      }
    case "timestamp":
    case "datetime":
      return {
        name: ckanField.id,
        type: "date-time",
        property: { ...baseProperty, type: "string", format: "date-time" },
      }
    case "json":
    case "object":
      return {
        name: ckanField.id,
        type: "object",
        property: { ...baseProperty, type: "object" },
      }
    case "array":
      return {
        name: ckanField.id,
        type: "array",
        property: { ...baseProperty, type: "array" },
      }
    default:
      return {
        name: ckanField.id,
        type: "string",
        property: { ...baseProperty, type: "string" },
      }
  }
}
