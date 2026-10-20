import type { Resource } from "@fairspec/library"
import { describe, expect, it } from "vite-plus/test"
import { validateTableTool } from "../validate.ts"

describe("validateTableTool", () => {
  it("validates tool structure", () => {
    expect(validateTableTool.name).toBe("validateTableTool")
    expect(validateTableTool.description).toBeTruthy()
    expect(validateTableTool.inputSchema).toBeTruthy()
    expect(validateTableTool.outputSchema).toBeTruthy()
    expect(validateTableTool.execute).toBeTypeOf("function")
  })

  it("validates a valid table resource", async () => {
    const validResource: Resource = {
      data: [
        { name: "Alice", age: 30 },
        { name: "Bob", age: 25 },
      ],
      tableSchema: {
        properties: {
          name: { type: "string" as const },
          age: { type: "integer" as const },
        },
      },
    }

    const result = await validateTableTool.execute({
      resource: validResource,
    })

    expect(result.report.valid).toBe(true)
    expect(result.report.errors).toHaveLength(0)
  })

  it("detects invalid data in table", async () => {
    const invalidResource: Resource = {
      data: [
        { name: "Alice", age: "not a number" },
        { name: "Bob", age: 25 },
      ],
      tableSchema: {
        properties: {
          name: { type: "string" as const },
          age: { type: "integer" as const },
        },
      },
    }

    const result = await validateTableTool.execute({
      resource: invalidResource,
    })

    expect(result.report.valid).toBe(false)
    expect(result.report.errors.length).toBeGreaterThan(0)
    expect(result.report.errors[0]?.type).toContain("cell")
  })

  it("respects maxErrors option", async () => {
    const invalidResource: Resource = {
      data: [
        { age: "invalid1" },
        { age: "invalid2" },
        { age: "invalid3" },
        { age: "invalid4" },
        { age: "invalid5" },
      ],
      tableSchema: {
        properties: {
          age: { type: "integer" as const },
        },
      },
    }

    const result = await validateTableTool.execute({
      resource: invalidResource,
      options: { maxErrors: 2 },
    })

    expect(result.report.valid).toBe(false)
    expect(result.report.errors.length).toBeLessThanOrEqual(2)
  })

  it("infers schema when not provided", async () => {
    const resourceWithoutSchema: Resource = {
      data: [
        { name: "Alice", age: 30 },
        { name: "Bob", age: 25 },
      ],
    }

    const result = await validateTableTool.execute({
      resource: resourceWithoutSchema,
    })

    expect(result.report.valid).toBe(true)
    expect(result.report.errors).toHaveLength(0)
  })

  it("skips inference when noInfer is true", async () => {
    const resourceWithoutSchema: Resource = {
      data: [{ name: "Alice" }],
    }

    const result = await validateTableTool.execute({
      resource: resourceWithoutSchema,
      options: { noInfer: true },
    })

    expect(result.report.valid).toBe(true)
  })

  it("returns at most 100 errors by default", async () => {
    const resource: Resource = {
      data: Array.from({ length: 300 }, () => ({ age: "invalid" })),
      tableSchema: { properties: { age: { type: "integer" as const } } },
    }

    const result = await validateTableTool.execute({ resource })

    expect(result.report.valid).toBe(false)
    expect(result.report.errors).toHaveLength(100)
    expect(result).toMatchObject({ truncated: true })
  })

  it("does not mark a complete report as truncated", async () => {
    const resource: Resource = {
      data: [{ age: "invalid" }, { age: 1 }],
      tableSchema: { properties: { age: { type: "integer" as const } } },
    }

    const result = await validateTableTool.execute({ resource })

    expect(result.report.errors).toHaveLength(1)
    expect(result).toMatchObject({ truncated: false })
  })

  it("fills maxErrors from one bad column of many", async () => {
    const resource: Resource = {
      data: Array.from({ length: 300 }, (_, i) => ({ id: i, age: "invalid" })),
      tableSchema: {
        properties: {
          id: { type: "integer" as const },
          age: { type: "integer" as const },
        },
      },
    }

    const result = await validateTableTool.execute({ resource })

    expect(result.report.errors).toHaveLength(100)
    expect(result.truncated).toBe(true)
  })
})
