import { convertFieldNames } from "../../actions/frictionless/list.ts"
import type { FrictionlessRecorder } from "../../actions/frictionless/mapping.ts"
import { convertNameFromFrictionless } from "../../actions/frictionless/name.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { FrictionlessField } from "../../models/frictionless/field.ts"
import type { FrictionlessSchema } from "../../models/frictionless/schema.ts"

const FIELD_KEYS: Record<string, string> = {
  type: "type",
  title: "title",
  description: "description",
  rdfType: "rdfType",
  examples: "example",
  missingValues: "missingValues",
  enum: "constraints/enum",
  minimum: "constraints/minimum",
  maximum: "constraints/maximum",
  exclusiveMinimum: "constraints/exclusiveMinimum",
  exclusiveMaximum: "constraints/exclusiveMaximum",
  minLength: "constraints/minLength",
  maxLength: "constraints/maxLength",
  minItems: "constraints/minLength",
  maxItems: "constraints/maxLength",
  pattern: "constraints/pattern",
  decimalChar: "decimalChar",
  groupChar: "groupChar",
  withText: "bareNumber",
  trueValues: "trueValues",
  falseValues: "falseValues",
  categories: "categories",
  withOrder: "categoriesOrdered",
  delimiter: "delimiter",
  itemType: "itemType",
  temporalFormat: "format",
}
const VERBATIM_KEYS = ["missingValues", "enum", "trueValues", "falseValues", "categories"]

const STRING_TYPES = ["string", "email", "url", "base64", "categorical", "yearmonth"]
const NUMERIC_TYPES = ["integer", "number", "year"]
const ITEM_TYPES: Record<string, string> = {
  string: "string",
  integer: "integer",
  number: "number",
  boolean: "boolean",
  date: "date",
  time: "local-time",
  datetime: "date-time",
}

/**
 * Convert a Frictionless Table Schema to a Fairspec Table Schema descriptor
 */
export function convertTableSchemaFromFrictionless(
  schema: FrictionlessSchema,
  options?: { record?: FrictionlessRecorder },
) {
  const record: FrictionlessRecorder = options?.record ?? (() => {})
  const tableSchema: Descriptor = {}
  const properties: Record<string, Descriptor> = {}
  const uniqueKeys = [...(schema.uniqueKeys ?? [])]

  for (const [index, field] of schema.fields.entries()) {
    const property = convertField(field)
    properties[field.name] = property

    const fairspec = `/properties/${escapePointerKey(field.name)}`
    const frictionless = `/fields/${index}`
    record(fairspec, frictionless)
    for (const key of Object.keys(property)) {
      const target = FIELD_KEYS[key]
      if (!target) continue
      record(`${fairspec}/${key}`, `${frictionless}/${target}`, {
        verbatim: VERBATIM_KEYS.includes(key),
      })
    }

    if (field.constraints?.unique) {
      record(`/uniqueKeys/${uniqueKeys.length}`, `${frictionless}/constraints/unique`)
      uniqueKeys.push([field.name])
    }
  }

  record("/properties", "/fields")
  record("/missingValues", "/missingValues", { verbatim: true })
  record("/primaryKey", "/primaryKey")
  for (const index of (schema.uniqueKeys ?? []).keys()) {
    record(`/uniqueKeys/${index}`, `/uniqueKeys/${index}`, { verbatim: true })
  }
  for (const index of (schema.foreignKeys ?? []).keys()) {
    const prefix = `/foreignKeys/${index}`
    record(prefix, prefix)
    record(`${prefix}/columns`, `${prefix}/fields`)
    record(`${prefix}/reference`, `${prefix}/reference`)
    record(`${prefix}/reference/resource`, `${prefix}/reference/resource`)
    record(`${prefix}/reference/columns`, `${prefix}/reference/fields`)
  }

  tableSchema.properties = properties

  // Fairspec formats already read an empty cell as null, Frictionless's default `[""]`
  const missingValues = schema.missingValues ?? []
  const isDefault = missingValues.length === 1 && missingValues[0] === ""
  if (missingValues.length > 0 && !isDefault) tableSchema.missingValues = missingValues

  if (schema.primaryKey) {
    const primaryKey = convertFieldNames(schema.primaryKey)
    if (primaryKey.length > 0) tableSchema.primaryKey = primaryKey
  }

  if (uniqueKeys.length > 0) tableSchema.uniqueKeys = uniqueKeys

  if (schema.foreignKeys && schema.foreignKeys.length > 0) {
    tableSchema.foreignKeys = schema.foreignKeys.map(foreignKey => {
      const reference: Descriptor = {}
      if (foreignKey.reference.resource) {
        reference.resource = convertNameFromFrictionless(foreignKey.reference.resource)
      }
      reference.columns = convertFieldNames(foreignKey.reference.fields)
      return { columns: convertFieldNames(foreignKey.fields), reference }
    })
  }

  return tableSchema
}

function convertField(field: FrictionlessField) {
  const constraints = field.constraints ?? {}
  const { kind, property } = convertType(field)

  if (field.title) property.title = field.title
  if (field.description) property.description = field.description
  if (field.rdfType) property.rdfType = field.rdfType
  if (field.example !== undefined) property.examples = [field.example]
  if (field.missingValues && field.missingValues.length > 0) {
    property.missingValues = field.missingValues
  }

  if (constraints.enum) property.enum = constraints.enum

  if (NUMERIC_TYPES.includes(kind)) {
    for (const key of ["minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum"]) {
      const value = constraints[key]
      if (typeof value === "number") property[key] = value
    }
  }

  if (STRING_TYPES.includes(kind)) {
    if (constraints.minLength !== undefined) property.minLength = constraints.minLength
    if (constraints.maxLength !== undefined) property.maxLength = constraints.maxLength
    if (constraints.pattern) property.pattern = constraints.pattern
  }

  if (kind === "list") {
    if (constraints.minLength !== undefined) property.minItems = constraints.minLength
    if (constraints.maxLength !== undefined) property.maxItems = constraints.maxLength
  }

  if ((kind === "object" || kind === "array") && constraints.jsonSchema) {
    Object.assign(property, constraints.jsonSchema)
  }

  if (!constraints.required && typeof property.type === "string") {
    property.type = [property.type, "null"]
  }

  return property
}

function convertType(field: FrictionlessField): { kind: string; property: Descriptor } {
  const format = field.format
  const temporalFormat = convertTemporalFormat(format)

  switch (field.type ?? "string") {
    case "string": {
      if (field.categories) {
        return { kind: "categorical", property: convertCategorical("string", field) }
      }
      if (format === "email") {
        return { kind: "email", property: { type: "string", format: "email" } }
      }
      if (format === "uri") {
        return { kind: "url", property: { type: "string", format: "url" } }
      }
      if (format === "binary") {
        return { kind: "base64", property: { type: "string", format: "base64" } }
      }
      return { kind: "string", property: { type: "string" } }
    }
    case "integer": {
      if (field.categories) {
        return { kind: "categorical", property: convertCategorical("integer", field) }
      }
      const property: Descriptor = { type: "integer" }
      if (field.groupChar) property.groupChar = field.groupChar
      if (field.bareNumber === false) property.withText = true
      return { kind: "integer", property }
    }
    case "number": {
      const property: Descriptor = { type: "number" }
      if (field.decimalChar) property.decimalChar = field.decimalChar
      if (field.groupChar) property.groupChar = field.groupChar
      if (field.bareNumber === false) property.withText = true
      return { kind: "number", property }
    }
    case "boolean": {
      const property: Descriptor = { type: "boolean" }
      if (field.trueValues) property.trueValues = field.trueValues
      if (field.falseValues) property.falseValues = field.falseValues
      return { kind: "boolean", property }
    }
    case "date":
      return { kind: "date", property: createTemporal("date", temporalFormat) }
    case "time": {
      const withOffset = temporalFormat?.includes("z")
      return {
        kind: "time",
        property: createTemporal(withOffset ? "time" : "local-time", temporalFormat),
      }
    }
    case "datetime": {
      const isLocal = temporalFormat !== undefined && !temporalFormat.includes("z")
      return {
        kind: "datetime",
        property: createTemporal(
          isLocal ? "local-date-time" : "date-time",
          temporalFormat,
        ),
      }
    }
    case "year":
      return { kind: "year", property: { type: "integer" } }
    case "yearmonth":
      return {
        kind: "yearmonth",
        property: { type: "string", pattern: "^\\d{4}-\\d{2}$" },
      }
    case "duration":
      return { kind: "duration", property: { type: "string", format: "duration" } }
    case "geopoint":
      if (format === "array") return { kind: "array", property: { type: "array" } }
      if (format === "object") return { kind: "object", property: { type: "object" } }
      return { kind: "string", property: { type: "string" } }
    case "geojson":
      return {
        kind: "geojson",
        property: {
          type: "object",
          format: format === "topojson" ? "topojson" : "geojson",
        },
      }
    case "object":
      return { kind: "object", property: { type: "object" } }
    case "array":
      return { kind: "array", property: { type: "array" } }
    case "list": {
      const property: Descriptor = { type: "string", format: "list" }
      if (field.delimiter) property.delimiter = field.delimiter
      const itemType = field.itemType ? ITEM_TYPES[field.itemType] : undefined
      if (itemType) property.itemType = itemType
      return { kind: "list", property }
    }
    default:
      return { kind: "unknown", property: {} }
  }
}

function convertCategorical(type: "string" | "integer", field: FrictionlessField) {
  const property: Descriptor = {
    type,
    format: "categorical",
    categories: field.categories,
  }
  if (field.categoriesOrdered) property.withOrder = true
  return property
}

function createTemporal(format: string, temporalFormat?: string) {
  const property: Descriptor = { type: "string", format }
  if (temporalFormat) property.temporalFormat = temporalFormat
  return property
}

function convertTemporalFormat(format?: string) {
  if (!format || format === "default" || format === "any") return undefined
  return format.startsWith("fmt:") ? format.slice(4) : format
}

function escapePointerKey(key: string) {
  return key.replaceAll("~", "~0").replaceAll("/", "~1")
}
