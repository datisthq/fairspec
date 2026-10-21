import { describe, expect, it } from "vite-plus/test"
import { convertTableSchema as convertToCsvSchema } from "../tools/csv-validator.ts"
import { TOOLS, selectTools } from "../tools/index.ts"
import { convertTableSchema } from "../tools/qsv.ts"

describe("convertTableSchema", () => {
  it("maps nullability, local-date-time and the primary key to JSON Schema", () => {
    const schema = convertTableSchema({
      properties: {
        id: { type: "integer", minimum: 1 },
        at: { type: ["string", "null"], format: "local-date-time" },
        zip: { type: ["string", "null"], pattern: "^[0-9]{5}$" },
      },
      required: ["id"],
      primaryKey: ["id"],
    })
    expect(schema.properties).toEqual({
      id: { type: "integer", minimum: 1 },
      at: {
        type: ["string", "null"],
        pattern: "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}$",
      },
      zip: { type: ["string", "null"], pattern: "^[0-9]{5}$" },
    })
    expect(schema.uniqueCombinedWith).toEqual(["id"])
  })
})

describe("convertToCsvSchema", () => {
  it("maps types, constraints, nullability and the primary key to CSV Schema", () => {
    const schema = convertToCsvSchema({
      properties: {
        id: { type: "integer", minimum: 1 },
        status: { type: "string", enum: ["Open", "Closed"] },
        zip: { type: ["string", "null"], pattern: "[0-9]{5}" },
      },
      required: ["id", "status"],
      primaryKey: ["id"],
    })
    expect(schema.split("\n")).toEqual([
      "version 1.1",
      "@totalColumns 3",
      'id: notEmpty regex("^[+-]?[0-9]+$") range(1,*) unique',
      'status: notEmpty any("Open","Closed")',
      'zip: regex("(?s).*(?:[0-9]{5}).*") @optional',
      "",
    ])
  })
})

describe("selectTools", () => {
  it("returns every tool by default and rejects unknown names", () => {
    expect(selectTools()).toBe(TOOLS)
    expect(selectTools(["qsv"]).map(tool => tool.name)).toEqual(["qsv"])
    expect(() => selectTools(["csvkit"])).toThrow(/Unknown tools csvkit/)
  })
})
