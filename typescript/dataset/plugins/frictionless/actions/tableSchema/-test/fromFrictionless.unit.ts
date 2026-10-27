import { describe, expect, it } from "vite-plus/test"
import { convertTableSchemaFromFrictionless } from "../fromFrictionless.ts"

describe("convertTableSchemaFromFrictionless", () => {
  it("should allow null unless a field is required", () => {
    const tableSchema = convertTableSchemaFromFrictionless({
      fields: [
        { name: "id", type: "integer", constraints: { required: true } },
        { name: "name", type: "string" },
      ],
    })

    expect(tableSchema.properties).toEqual({
      id: { type: "integer" },
      name: { type: ["string", "null"] },
    })
  })

  it("should drop the default missing values", () => {
    expect(
      convertTableSchemaFromFrictionless({ fields: [], missingValues: [""] })
        .missingValues,
    ).toBeUndefined()
    expect(
      convertTableSchemaFromFrictionless({ fields: [], missingValues: ["", "NA"] })
        .missingValues,
    ).toEqual(["", "NA"])
  })

  it("should read a datetime pattern without an offset as local", () => {
    const tableSchema = convertTableSchemaFromFrictionless({
      fields: [
        { name: "a", type: "datetime" },
        { name: "b", type: "datetime", format: "%Y-%m-%d %H:%M" },
        { name: "c", type: "datetime", format: "%Y-%m-%dT%H:%M%z" },
      ],
    })

    expect(tableSchema.properties).toEqual({
      a: { type: ["string", "null"], format: "date-time" },
      b: {
        type: ["string", "null"],
        format: "local-date-time",
        temporalFormat: "%Y-%m-%d %H:%M",
      },
      c: {
        type: ["string", "null"],
        format: "date-time",
        temporalFormat: "%Y-%m-%dT%H:%M%z",
      },
    })
  })

  it("should rename referenced resources and resolve a self-reference", () => {
    const tableSchema = convertTableSchemaFromFrictionless({
      fields: [{ name: "id" }, { name: "parent" }],
      foreignKeys: [
        { fields: "parent", reference: { resource: "", fields: "id" } },
        { fields: ["id"], reference: { resource: "other-table", fields: ["id"] } },
      ],
    })

    expect(tableSchema.foreignKeys).toEqual([
      { columns: ["parent"], reference: { columns: ["id"] } },
      { columns: ["id"], reference: { resource: "other_table", columns: ["id"] } },
    ])
  })
})
