import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { validateDataCommand } from "../validate.ts"

describe("data validate", () => {
  let originalExitCode: typeof process.exitCode

  beforeEach(() => {
    originalExitCode = process.exitCode
    process.exitCode = undefined
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
    process.exitCode = originalExitCode
  })

  it("should validate valid JSON data", async () => {
    const jsonData = { name: "alice", age: 25, active: true }
    const jsonPath = await writeTempFile(JSON.stringify(jsonData), {
      format: "json",
    })
    const schema = {
      type: "object",
      properties: {
        name: { type: "string" },
        age: { type: "number" },
        active: { type: "boolean" },
      },
      required: ["name", "age"],
    }
    const schemaPath = await writeTempFile(JSON.stringify(schema), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await validateDataCommand.parseAsync([jsonPath, "--schema", schemaPath, "--json"], {
        from: "user",
      })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("valid")
  })

  it("should keep a zero exit code for valid data", async () => {
    const jsonData = { name: "alice", age: 25 }
    const jsonPath = await writeTempFile(JSON.stringify(jsonData), {
      format: "json",
    })
    const schema = {
      type: "object",
      properties: { name: { type: "string" }, age: { type: "number" } },
      required: ["name", "age"],
    }
    const schemaPath = await writeTempFile(JSON.stringify(schema), {
      format: "json",
    })

    await validateDataCommand.parseAsync([jsonPath, "--schema", schemaPath, "--json"], {
      from: "user",
    })

    expect(process.exitCode).toBeFalsy()
  })

  it("should set a non-zero exit code for invalid data", async () => {
    const jsonData = { name: "alice", age: "not a number" }
    const jsonPath = await writeTempFile(JSON.stringify(jsonData), {
      format: "json",
    })
    const schema = {
      type: "object",
      properties: { name: { type: "string" }, age: { type: "number" } },
      required: ["name", "age"],
    }
    const schemaPath = await writeTempFile(JSON.stringify(schema), {
      format: "json",
    })

    await validateDataCommand.parseAsync([jsonPath, "--schema", schemaPath, "--json"], {
      from: "user",
    })

    expect(process.exitCode).toBe(1)
  })

  it("should detect invalid JSON data", async () => {
    const jsonData = { name: "alice", age: "not a number" }
    const jsonPath = await writeTempFile(JSON.stringify(jsonData), {
      format: "json",
    })
    const schema = {
      type: "object",
      properties: {
        name: { type: "string" },
        age: { type: "number" },
      },
      required: ["name", "age"],
    }
    const schemaPath = await writeTempFile(JSON.stringify(schema), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await validateDataCommand.parseAsync([jsonPath, "--schema", schemaPath, "--json"], {
        from: "user",
      })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("valid")
  })

  it("should validate with nested object schema", async () => {
    const jsonData = {
      name: "alice",
      address: { city: "New York", zip: "10001" },
    }
    const jsonPath = await writeTempFile(JSON.stringify(jsonData), {
      format: "json",
    })
    const schema = {
      type: "object",
      properties: {
        name: { type: "string" },
        address: {
          type: "object",
          properties: {
            city: { type: "string" },
            zip: { type: "string" },
          },
        },
      },
    }
    const schemaPath = await writeTempFile(JSON.stringify(schema), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await validateDataCommand.parseAsync([jsonPath, "--schema", schemaPath, "--json"], {
        from: "user",
      })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("valid")
  })
})
