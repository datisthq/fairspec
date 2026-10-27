import { describe, expect, it } from "vite-plus/test"
import { FileDialect } from "../../../models/fileDialect/fileDialect.ts"
import { Resource } from "../../../models/resource.ts"
import { validateFileDialect } from "../validate.ts"

describe("validateFileDialect", () => {
  it("accepts a builtin format", async () => {
    const report = await validateFileDialect({ format: "csv", delimiter: ";" })

    expect(report.valid).toBe(true)
    expect(report.fileDialect).toEqual({ format: "csv", delimiter: ";" })
  })

  it("accepts a custom format", async () => {
    const report = await validateFileDialect({ format: "custom", title: "Custom format" })

    expect(report.valid).toBe(true)
    expect(report.errors).toEqual([])
    expect(report.fileDialect).toEqual({ format: "custom", title: "Custom format" })
  })

  it("rejects a missing format", async () => {
    const report = await validateFileDialect({ title: "Custom format" })

    expect(report.valid).toBe(false)
  })
})

describe("FileDialect", () => {
  it("parses a custom format in a resource", () => {
    const resource = Resource.parse({
      data: "file.bin",
      fileDialect: { format: "custom" },
    })

    expect(resource.fileDialect).toEqual({ format: "custom" })
  })

  it("checks the properties of a builtin format instead of reading it as custom", () => {
    const result = FileDialect.safeParse({ format: "csv", delimiter: 5 })

    expect(result.success).toBe(false)
  })
})
