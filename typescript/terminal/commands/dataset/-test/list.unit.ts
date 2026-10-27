import { basename } from "node:path"
import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { listDatasetCommand } from "../list.ts"

describe("dataset list", () => {
  beforeEach(() => {
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("should list resources from a dataset", async () => {
    const csvPath = await writeTempFile("id,name\n1,alice\n2,bob", {
      format: "csv",
    })
    const descriptor = {
      resources: [{ name: "users", data: basename(csvPath) }],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await listDatasetCommand.parseAsync([descriptorPath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(Array.isArray(data)).toBe(true)
    expect(data).toContain("users")
  })

  it("should list multiple resources", async () => {
    const csv1Path = await writeTempFile("id,name\n1,alice", {
      format: "csv",
    })
    const csv2Path = await writeTempFile("id,age\n1,25", { format: "csv" })
    const descriptor = {
      resources: [
        { name: "users", data: basename(csv1Path) },
        { name: "ages", data: basename(csv2Path) },
      ],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await listDatasetCommand.parseAsync([descriptorPath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveLength(2)
    expect(data).toContain("users")
    expect(data).toContain("ages")
  })

  it("should infer resource names when not provided", async () => {
    const csvPath = await writeTempFile("id,name\n1,alice", {
      format: "csv",
    })
    const descriptor = {
      resources: [{ data: basename(csvPath) }],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await listDatasetCommand.parseAsync([descriptorPath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(Array.isArray(data)).toBe(true)
    expect(data).toHaveLength(1)
  })
})
