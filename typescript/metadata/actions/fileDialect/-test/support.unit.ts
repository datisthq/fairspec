import { describe, expect, it } from "vite-plus/test"
import type { Resource } from "../../../models/resource.ts"
import { getSupportedFileDialect } from "../support.ts"

describe("getSupportedFileDialect", () => {
  it("returns a declared builtin format", async () => {
    const resource: Resource = { data: "table.txt", fileDialect: { format: "csv" } }

    expect(await getSupportedFileDialect(resource, ["csv"])).toEqual({ format: "csv" })
  })

  it("infers the format without a dialect", async () => {
    expect(await getSupportedFileDialect({ data: "table.csv" }, ["csv"])).toEqual({
      format: "csv",
    })
  })

  it("never infers the format for a custom dialect", async () => {
    const resource: Resource = { data: "table.csv", fileDialect: { format: "custom" } }

    expect(await getSupportedFileDialect(resource, ["csv"])).toBe(undefined)
  })
})
