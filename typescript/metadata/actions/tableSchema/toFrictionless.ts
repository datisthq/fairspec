import { getBasePropertyType } from "../../actions/column/property.ts"
import type { Column } from "../../models/column/column.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import { getColumns } from "./column.ts"

const ITEM_TYPES: Record<string, string> = {
  string: "string",
  integer: "integer",
  number: "number",
  boolean: "boolean",
  date: "date",
  time: "time",
  "local-time": "time",
  "date-time": "datetime",
  "local-date-time": "datetime",
}

/**
 * Convert a Fairspec Table Schema to a Frictionless Table Schema descriptor
 */
export function convertTableSchemaToFrictionless(tableSchema: TableSchema) {
  const schema: Descriptor = {}

  schema.fields = getColumns(tableSchema).map(column => convertColumn(column))

  // Fairspec formats read an empty cell as null, which Frictionless does only through `""`
  const missingValues = [...(tableSchema.missingValues ?? [])]
  if (!missingValues.includes("")) missingValues.push("")
  const isDefault = missingValues.length === 1
  if (!isDefault) schema.missingValues = missingValues

  if (tableSchema.primaryKey) schema.primaryKey = tableSchema.primaryKey
  if (tableSchema.uniqueKeys) schema.uniqueKeys = tableSchema.uniqueKeys

  if (tableSchema.foreignKeys) {
    schema.foreignKeys = tableSchema.foreignKeys.map(foreignKey => ({
      fields: foreignKey.columns,
      reference: {
        resource: foreignKey.reference.resource ?? "",
        fields: foreignKey.reference.columns,
      },
    }))
  }

  return schema
}

function convertColumn(column: Column) {
  const field: Descriptor = { name: column.name, ...convertType(column) }
  const constraints: Descriptor = {}
  const { property } = column

  if (property.title) field.title = property.title
  if (property.description) field.description = property.description
  if (property.rdfType) field.rdfType = property.rdfType

  if (!column.nullable && column.type !== "unknown") constraints.required = true

  switch (column.type) {
    case "integer":
    case "number":
    case "decimal":
      for (const key of [
        "minimum",
        "maximum",
        "exclusiveMinimum",
        "exclusiveMaximum",
      ] as const) {
        const value = column.property[key]
        if (value !== undefined) constraints[key] = value
      }
      if (column.property.groupChar) field.groupChar = column.property.groupChar
      if (column.property.withText) field.bareNumber = false
      if (column.type !== "integer" && column.property.decimalChar) {
        field.decimalChar = column.property.decimalChar
      }
      break
    case "string":
      if (column.property.minLength !== undefined) {
        constraints.minLength = column.property.minLength
      }
      if (column.property.maxLength !== undefined) {
        constraints.maxLength = column.property.maxLength
      }
      if (column.property.pattern) constraints.pattern = column.property.pattern
      break
    case "boolean":
      if (column.property.trueValues) field.trueValues = column.property.trueValues
      if (column.property.falseValues) field.falseValues = column.property.falseValues
      break
    case "list":
      if (column.property.delimiter) field.delimiter = column.property.delimiter
      if (column.property.itemType) field.itemType = ITEM_TYPES[column.property.itemType]
      if (column.property.minItems !== undefined) {
        constraints.minLength = column.property.minItems
      }
      if (column.property.maxItems !== undefined) {
        constraints.maxLength = column.property.maxItems
      }
      break
    case "categorical":
      field.categories = column.property.categories
      if (column.property.withOrder) field.categoriesOrdered = true
      break
    case "date":
    case "time":
    case "local-time":
    case "date-time":
    case "local-date-time":
      if (column.property.temporalFormat) field.format = column.property.temporalFormat
      break
  }

  if ("enum" in property && Array.isArray(property.enum)) {
    constraints.enum = property.enum.filter(value => value !== null)
  }

  if (Object.keys(constraints).length > 0) field.constraints = constraints

  return field
}

function convertType(column: Column): Descriptor {
  switch (column.type) {
    case "string":
    case "hex":
    case "wkt":
    case "wkb":
      return { type: "string" }
    case "email":
      return { type: "string", format: "email" }
    case "url":
      return { type: "string", format: "uri" }
    case "base64":
      return { type: "string", format: "binary" }
    case "categorical":
      return {
        type:
          getBasePropertyType(column.property.type) === "integer" ? "integer" : "string",
      }
    case "integer":
      return { type: "integer" }
    case "number":
    case "decimal":
      return { type: "number" }
    case "boolean":
      return { type: "boolean" }
    case "date":
      return { type: "date" }
    case "time":
    case "local-time":
      return { type: "time" }
    case "date-time":
    case "local-date-time":
      return { type: "datetime" }
    case "duration":
      return { type: "duration" }
    case "list":
      return { type: "list" }
    case "array":
      return { type: "array" }
    case "object":
      return { type: "object" }
    case "geojson":
      return { type: "geojson" }
    case "topojson":
      return { type: "geojson", format: "topojson" }
    default:
      return { type: "any" }
  }
}
