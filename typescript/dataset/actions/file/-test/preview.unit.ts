import { Buffer } from "node:buffer"
import { describe, expect, it } from "vite-plus/test"
import { writeTempFile } from "../../../actions/file/temp.ts"
import { previewFile } from "../preview.ts"

function createLines(count: number) {
  return Array.from({ length: count }, (_, i) => `${i},row${i}\n`)
}

describe("previewFile", () => {
  it("should return a small file whole", async () => {
    const path = await writeTempFile("id,name\n1,alice\n")

    const preview = await previewFile(path)

    expect(preview).toEqual({ text: "id,name\n1,alice\n", truncated: false })
  })

  it("should keep the first 20 lines by default", async () => {
    const lines = createLines(50)
    const path = await writeTempFile(lines.join(""))

    const preview = await previewFile(path)

    expect(preview).toEqual({ text: lines.slice(0, 20).join(""), truncated: true })
  })

  it("should cut at maxLines", async () => {
    const lines = createLines(10)
    const path = await writeTempFile(lines.join(""))

    const preview = await previewFile(path, { maxLines: 3 })

    expect(preview).toEqual({ text: lines.slice(0, 3).join(""), truncated: true })
  })

  it("should cut at maxBytes before maxLines", async () => {
    const path = await writeTempFile("a".repeat(100))

    const preview = await previewFile(path, { maxBytes: 10 })

    expect(preview).toEqual({ text: "a".repeat(10), truncated: true })
  })

  it("should not report a file of exactly maxBytes as truncated", async () => {
    const path = await writeTempFile("a".repeat(10))

    const preview = await previewFile(path, { maxBytes: 10 })

    expect(preview).toEqual({ text: "a".repeat(10), truncated: false })
  })

  it("should not report a file of exactly maxLines as truncated", async () => {
    const lines = createLines(3)
    const path = await writeTempFile(lines.join(""))

    const preview = await previewFile(path, { maxLines: 3 })

    expect(preview).toEqual({ text: lines.join(""), truncated: false })
  })

  it("should drop a multi-byte character cut by maxBytes", async () => {
    const path = await writeTempFile(`${"a".repeat(99)}é`)

    const preview = await previewFile(path, { maxBytes: 100 })

    expect(preview).toEqual({ text: "a".repeat(99), truncated: true })
  })

  it("should reject non-positive limits", async () => {
    const path = await writeTempFile("id\n")

    await expect(previewFile(path, { maxBytes: 0 })).rejects.toThrow(
      "maxBytes must be a positive integer",
    )
    await expect(previewFile(path, { maxLines: -1 })).rejects.toThrow(
      "maxLines must be a positive integer",
    )
  })

  it("should reject a binary file", async () => {
    const path = await writeTempFile(Buffer.from([0x00, 0x01, 0x02, 0x03, 0xff, 0x00]))

    await expect(previewFile(path)).rejects.toThrow("Cannot preview a binary file")
  })
})
