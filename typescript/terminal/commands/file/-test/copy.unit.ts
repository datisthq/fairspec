import { existsSync } from "node:fs"
import { readFile } from "node:fs/promises"
import { getTempFilePath, writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { copyFileCommand } from "../copy.ts"

describe("file copy", () => {
  beforeEach(() => {
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("should copy a file", async () => {
    const sourcePath = await writeTempFile("test content", { format: "txt" })
    const targetPath = getTempFilePath()

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await copyFileCommand.parseAsync([sourcePath, "--to-path", targetPath, "--json"], {
        from: "user",
      })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("result")
    expect(existsSync(targetPath)).toBe(true)
    const content = await readFile(targetPath, "utf-8")
    expect(content).toBe("test content")
  })

  it("should copy a CSV file", async () => {
    const csvContent = "id,name,age\n1,alice,25\n2,bob,30"
    const sourcePath = await writeTempFile(csvContent, { format: "csv" })
    const targetPath = getTempFilePath()

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await copyFileCommand.parseAsync([sourcePath, "--to-path", targetPath, "--json"], {
        from: "user",
      })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    expect(existsSync(targetPath)).toBe(true)
    const content = await readFile(targetPath, "utf-8")
    expect(content).toBe(csvContent)
  })

  it("should copy a JSON file", async () => {
    const jsonContent = JSON.stringify({ id: 1, name: "test" })
    const sourcePath = await writeTempFile(jsonContent, { format: "json" })
    const targetPath = getTempFilePath()

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await copyFileCommand.parseAsync([sourcePath, "--to-path", targetPath, "--json"], {
        from: "user",
      })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    expect(existsSync(targetPath)).toBe(true)
    const content = await readFile(targetPath, "utf-8")
    expect(content).toBe(jsonContent)
  })
})
