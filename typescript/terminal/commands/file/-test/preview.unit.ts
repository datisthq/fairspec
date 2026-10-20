import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { previewFileCommand } from "../preview.ts"

describe("file preview", () => {
  let text: string[]

  beforeEach(() => {
    text = []
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  async function runCommand(args: string[]) {
    try {
      await previewFileCommand.parseAsync([...args], { from: "user" })
    } catch {}
  }

  it("should preview a CSV file as JSON", async () => {
    const path = await writeTempFile("id,name\n1,[alice]\n", { format: "csv" })

    await runCommand([path, "--json"])

    expect(JSON.parse(text[0] ?? "")).toEqual({
      text: "id,name\n1,[alice]\n",
      truncated: false,
    })
  })

  it("should limit the preview to --lines", async () => {
    const path = await writeTempFile("id\n1\n2\n3\n", { format: "csv" })

    await runCommand([path, "--lines", "2", "--json"])

    expect(JSON.parse(text[0] ?? "")).toEqual({ text: "id\n1\n", truncated: true })
  })

  it("should limit the preview to --sample-bytes", async () => {
    const path = await writeTempFile("abcdefgh", { format: "txt" })

    await runCommand([path, "--sample-bytes", "3", "--json"])

    expect(JSON.parse(text[0] ?? "")).toEqual({ text: "abc", truncated: true })
  })
})
