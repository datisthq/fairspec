import { z } from "zod"
import { describe, expect, it } from "vite-plus/test"
import { TableSchema } from "../../tableSchema.ts"
import { createColumnFromProperty } from "../../../actions/column/create.ts"
import { ColumnProperty } from "../column.ts"

describe("ColumnProperty", () => {
  it("accepts a property without a format", () => {
    for (const type of ["string", "integer", "number", "boolean", "object", "array"]) {
      expect(ColumnProperty.safeParse({ type }).success, type).toBe(true)
    }
  })

  it("accepts a property with a format supported by its type", () => {
    const properties = [
      { type: "string", format: "email" },
      { type: "string", format: "date-time" },
      { type: "string", format: "decimal" },
      { type: "string", format: "categorical" },
      { type: "integer", format: "categorical" },
      { type: "object", format: "geojson" },
    ]

    for (const property of properties) {
      expect(ColumnProperty.safeParse(property).success, property.format).toBe(true)
    }
  })

  it("preserves the format instead of stripping it", () => {
    const result = ColumnProperty.parse({ type: "string", format: "email" })
    expect(result).toEqual({ type: "string", format: "email" })
  })

  it("accepts a format that its type does not use, as an annotation", () => {
    const properties = [
      { type: "string", format: "uuid" },
      { type: "string", format: "geojson" },
      { type: "integer", format: "int64" },
      { type: "number", format: "categorical" },
      { type: "boolean", format: "categorical" },
      { type: "object", format: "bogus" },
      { type: "array", format: "list" },
    ]

    for (const property of properties) {
      expect(ColumnProperty.safeParse(property).success, property.format).toBe(true)
      expect(createColumnFromProperty("x", ColumnProperty.parse(property)).type).toBe(
        property.type,
      )
    }
  })
})

describe("TableSchema", () => {
  it("converts to JSON Schema for the MCP server", () => {
    const jsonSchema = z.toJSONSchema(TableSchema, { target: "draft-7", io: "output" })
    expect(jsonSchema).toBeTruthy()
  })
})
