import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { inferTableSchemaCommand } from "../infer.ts"

describe("schema infer", () => {
  beforeEach(() => {
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("should infer schema from csv table", async () => {
    const csvPath = await writeTempFile("id,name,age\n1,alice,25\n2,bob,30", {
      format: "csv",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await inferTableSchemaCommand.parseAsync([csvPath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("properties")
    expect(data.properties).toHaveProperty("id")
    expect(data.properties).toHaveProperty("name")
    expect(data.properties).toHaveProperty("age")
  })

  it("should infer schema with numeric types", async () => {
    const csvPath = await writeTempFile("id,value,score\n1,100,95.5\n2,200,87.3", {
      format: "csv",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await inferTableSchemaCommand.parseAsync([csvPath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("properties")
    expect(data.properties.id.type).toBe("integer")
    expect(data.properties.value.type).toBe("integer")
    expect(data.properties.score.type).toBe("number")
  })

  it("should infer schema with mixed types", async () => {
    const csvPath = await writeTempFile("name,active,count\nalice,true,5\nbob,false,10", {
      format: "csv",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await inferTableSchemaCommand.parseAsync([csvPath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("properties")
    expect(data.properties.name.type).toBe("string")
    expect(data.properties.active.type).toBe("boolean")
    expect(data.properties.count.type).toBe("integer")
  })

  it("should apply column properties over the inferred schema", async () => {
    const csvPath = await writeTempFile("id,price\n1,9.5\n2,7.25", { format: "csv" })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await inferTableSchemaCommand.parseAsync(
        [
          csvPath,
          "--column-properties",
          '{"id":{"type":"string"},"price":{"title":"Price"}}',
          "--json",
        ],
        { from: "user" },
      )
    } catch {}

    const data = JSON.parse(text[0] ?? "")
    expect(data.properties).toEqual({
      id: { type: "string" },
      price: { type: "number", title: "Price" },
    })
  })
})
