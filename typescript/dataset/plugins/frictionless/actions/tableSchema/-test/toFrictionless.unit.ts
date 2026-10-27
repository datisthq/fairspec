import { describe, expect, it } from "vite-plus/test"
import { convertTableSchemaToFrictionless } from "../toFrictionless.ts"

describe("convertTableSchemaToFrictionless", () => {
  it("should require a column that does not allow null", () => {
    const schema = convertTableSchemaToFrictionless({
      properties: {
        id: { type: "integer" },
        name: { type: ["string", "null"] },
      },
    })

    expect(schema.fields).toEqual([
      { name: "id", type: "integer", constraints: { required: true } },
      { name: "name", type: "string" },
    ])
  })

  it("should keep an empty cell missing", () => {
    expect(
      convertTableSchemaToFrictionless({ properties: {} }).missingValues,
    ).toBeUndefined()
    expect(
      convertTableSchemaToFrictionless({ properties: {}, missingValues: ["NA"] })
        .missingValues,
    ).toEqual(["NA", ""])
  })

  it("should write a self-reference as an empty resource", () => {
    const schema = convertTableSchemaToFrictionless({
      properties: { id: { type: "integer" }, parent: { type: "integer" } },
      foreignKeys: [{ columns: ["parent"], reference: { columns: ["id"] } }],
    })

    expect(schema.foreignKeys).toEqual([
      { fields: ["parent"], reference: { resource: "", fields: ["id"] } },
    ])
  })
})
