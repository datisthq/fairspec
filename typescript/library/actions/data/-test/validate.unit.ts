import type { Resource } from "@fairspec/metadata"
import { describe, expect, it } from "vite-plus/test"
import { validateData } from "../validate.ts"

describe("validateData", () => {
  it("should validate data against schema", async () => {
    const resource: Resource = {
      data: { name: "test", count: 5 },
      dataSchema: {
        type: "object",
        properties: {
          name: { type: "string" },
          count: { type: "integer" },
        },
      },
    }

    const report = await validateData(resource)
    expect(report.valid).toBe(true)
    expect(report.errors).toHaveLength(0)
  })

  it("should return errors for invalid data", async () => {
    const resource: Resource = {
      data: { name: 123, count: "invalid" },
      dataSchema: {
        type: "object",
        properties: {
          name: { type: "string" },
          count: { type: "integer" },
        },
      },
    }

    const report = await validateData(resource)
    expect(report.valid).toBe(false)
    expect(report.errors).toHaveLength(2)
    expect(report.errors).toContainEqual({
      type: "data",
      jsonPointer: "/name",
      message: "must be string",
    })
    expect(report.errors).toContainEqual({
      type: "data",
      jsonPointer: "/count",
      message: "must be integer",
    })
  })

  it("should return error when no data but schema expected", async () => {
    const resource: Resource = {
      name: "empty",
      dataSchema: { type: "object" },
    }

    const report = await validateData(resource)
    expect(report.valid).toBe(false)
    expect(report.errors).toHaveLength(1)
    expect(report.errors).toContainEqual({
      type: "resource/type",
      expectedResourceType: "data",
    })
  })

  it("should assert a format it knows", async () => {
    const report = await validateData({
      data: { day: "2023-02-29" },
      dataSchema: { properties: { day: { type: "string", format: "date" } } },
    })

    expect(report.errors).toEqual([
      { type: "data", message: expect.any(String), jsonPointer: "/day" },
    ])
  })

  it("should leave an unknown format an annotation", async () => {
    const report = await validateData({
      data: { site: "not a uri" },
      dataSchema: { properties: { site: { type: "string", format: "uri" } } },
    })

    expect(report).toEqual({ valid: true, errors: [] })
  })

  it("should report a remote $ref as metadata without fetching it", async () => {
    const report = await validateData({
      data: { a: 1 },
      dataSchema: { properties: { a: { $ref: "https://example.com/schema.json" } } },
    })

    expect(report.errors).toEqual([
      {
        type: "metadata",
        message: expect.any(String),
        jsonPointer: "/dataSchema/properties/a/$ref",
      },
    ])
  })
})
