import { describeFile, writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { validateFileCommand } from "../validate.ts"

describe("file validate", () => {
  beforeEach(() => {
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("should validate a file with correct hash", async () => {
    const content = "test content for validation"
    const filePath = await writeTempFile(content, { format: "txt" })

    const stats = await describeFile(filePath, { hashType: "md5" })
    const correctHash = stats.integrity?.hash ?? ""

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await validateFileCommand.parseAsync(
        [filePath, "--hash", correctHash, "--hash-type", "md5", "--json"],
        { from: "user" },
      )
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("valid")
    expect(data.valid).toBe(true)
  })

  it("should detect invalid hash", async () => {
    const filePath = await writeTempFile("test content", { format: "txt" })
    const wrongHash = "0000000000000000000000000000000"

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await validateFileCommand.parseAsync(
        [filePath, "--hash", wrongHash, "--hash-type", "md5", "--json"],
        { from: "user" },
      )
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("valid")
    expect(data.valid).toBe(false)
    expect(data.errors).toBeDefined()
    expect(data.errors.length).toBeGreaterThan(0)
  })

  it("should validate file without hash", async () => {
    const filePath = await writeTempFile("some content", { format: "txt" })

    const text: string[] = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })

    try {
      await validateFileCommand.parseAsync([filePath, "--json"], { from: "user" })
    } catch {}

    expect(text.length).toBeGreaterThan(0)
    const data = JSON.parse(text[0] ?? "")
    expect(data).toHaveProperty("valid")
    expect(data.valid).toBe(true)
  })
})
