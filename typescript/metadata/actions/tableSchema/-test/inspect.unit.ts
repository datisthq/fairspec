import { describe, expect, it } from "vite-plus/test"
import { inspectTableSchema, inspectTableSchemaNulls } from "../inspect.ts"

describe("inspectTableSchema", () => {
  it("reports a required column not in properties", () => {
    const errors = inspectTableSchema({
      required: ["id", "nope"],
      properties: { id: { type: "integer" } },
    })

    expect(errors).toEqual([
      {
        type: "metadata",
        message: 'column "nope" is not in properties',
        jsonPointer: "/required/1",
      },
    ])
  })

  it("reports a pattern that is not a regular expression", () => {
    const errors = inspectTableSchema({
      properties: { "a/b": { type: "string", pattern: "[a-" } },
    })

    expect(errors).toEqual([
      {
        type: "metadata",
        message: 'pattern "[a-" is not a valid regular expression',
        jsonPointer: "/properties/a~1b/pattern",
      },
    ])
  })

  it("accepts a valid pattern", () => {
    const errors = inspectTableSchema({
      properties: { code: { type: "string", pattern: "^[A-Z]{3}-[0-9]{4}$" } },
    })

    expect(errors).toEqual([])
  })
})

describe("inspectTableSchemaNulls", () => {
  it("reports null values on a column without null", () => {
    const errors = inspectTableSchemaNulls(
      {
        properties: {
          x: {
            type: "string",
            const: null,
            enum: ["a", null],
            default: null,
            examples: [null],
          },
        },
      },
      { rootJsonPointer: "/resources/0/tableSchema" },
    )

    const message = 'null is not allowed: the type of column "x" does not include "null"'
    expect(errors).toEqual([
      {
        type: "metadata",
        message,
        jsonPointer: "/resources/0/tableSchema/properties/x/enum/1",
      },
      {
        type: "metadata",
        message,
        jsonPointer: "/resources/0/tableSchema/properties/x/const",
      },
      {
        type: "metadata",
        message,
        jsonPointer: "/resources/0/tableSchema/properties/x/default",
      },
      {
        type: "metadata",
        message,
        jsonPointer: "/resources/0/tableSchema/properties/x/examples/0",
      },
    ])
  })

  it("accepts null values on a nullable or unknown column", () => {
    const errors = inspectTableSchemaNulls({
      properties: {
        x: { type: ["string", "null"], const: null, enum: ["a", null] },
        y: { const: null },
      },
    })

    expect(errors).toEqual([])
  })
})
