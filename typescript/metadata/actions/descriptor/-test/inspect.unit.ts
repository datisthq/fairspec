import { describe, expect, it } from "vite-plus/test"
import { TableSchema } from "../../../models/tableSchema.ts"
import { inspectModel } from "../inspect.ts"

describe("inspectModel", () => {
  it("returns the parsed model for a matching descriptor", () => {
    const descriptor = { properties: { id: { type: "integer" } }, "x-note": "kept" }

    const { data, errors } = inspectModel(descriptor, { model: TableSchema })

    expect(errors).toEqual([])
    expect(data).toEqual(descriptor)
  })

  it("reports a mismatch as a metadata error at its pointer", () => {
    const descriptor = { $schema: "http://", properties: {} }

    const { data, errors } = inspectModel(descriptor, {
      model: TableSchema,
      rootJsonPointer: "/resources/0/tableSchema",
    })

    expect(data).toBe(undefined)
    expect(errors).toEqual([
      expect.objectContaining({
        type: "metadata",
        jsonPointer: "/resources/0/tableSchema/$schema",
      }),
    ])
  })
})
