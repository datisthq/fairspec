import { describe, expect, it } from "vite-plus/test"
import { formatJsonPointer, parseJsonPointer } from "../pointer.ts"

describe("parseJsonPointer", () => {
  it("should parse the root pointer", () => {
    expect(parseJsonPointer("")).toEqual([])
  })

  it("should unescape reference tokens", () => {
    expect(parseJsonPointer("/data/a~1b/c~0d/~01")).toEqual(["data", "a/b", "c~d", "~1"])
  })

  it("should reject a pointer without a leading slash", () => {
    expect(() => parseJsonPointer("data")).toThrow()
  })
})

describe("formatJsonPointer", () => {
  it("should format the root pointer", () => {
    expect(formatJsonPointer([])).toEqual("")
  })

  it("should escape reference tokens", () => {
    expect(formatJsonPointer(["data", "a/b", "c~d", "~1"])).toEqual("/data/a~1b/c~0d/~01")
  })
})
