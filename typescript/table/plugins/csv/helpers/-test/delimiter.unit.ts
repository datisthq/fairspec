import { readFile } from "node:fs/promises"
import { writeTempFile } from "@fairspec/dataset"
import { describe, expect, it } from "vite-plus/test"
import { swapDelimiter } from "../delimiter.ts"

describe("swapDelimiter", () => {
  it("should keep a single-byte delimiter", async () => {
    const path = await writeTempFile("a;b\n")
    expect(await swapDelimiter([path], { format: "csv", delimiter: ";" })).toBeUndefined()
  })

  it("should swap a delimiter cut by a chunk boundary", async () => {
    const text = `${"a".repeat(65535)}¦b\n`
    const path = await writeTempFile(text)
    const swapped = await swapDelimiter([path], { format: "csv", delimiter: "¦" })

    expect(swapped?.swap).toEqual({ delimiter: "¦", placeholder: "\x1f" })
    expect(await readFile(swapped?.paths[0] ?? "", "utf8")).toBe(
      text.replace("¦", "\x1f"),
    )
  })

  it("should take a control byte the file does not hold", async () => {
    const path = await writeTempFile("a¦\x1f\n")
    const swapped = await swapDelimiter([path], { format: "csv", delimiter: "¦" })

    expect(swapped?.swap.placeholder).toBe("\x1e")
  })

  it("should read no further than maxBytes", async () => {
    const path = await writeTempFile("a¦b\n\x1f¦c\n")
    const swapped = await swapDelimiter(
      [path],
      { format: "csv", delimiter: "¦" },
      { maxBytes: 5 },
    )

    expect(swapped?.swap.placeholder).toBe("\x1f")
    expect(await readFile(swapped?.paths[0] ?? "", "utf8")).toBe("a\x1fb\n")
  })
})
