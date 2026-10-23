import { describe, expect, it } from "vite-plus/test"
import { parseDescriptor } from "../parse.ts"

describe("parseDescriptor", () => {
  it("parses an object", () => {
    expect(parseDescriptor('{"name": "test"}')).toEqual({ name: "test" })
  })

  it.each([
    ['["secret", "token"]', "array"],
    ['"secret token"', "string"],
    ["42", "number"],
    ["1.5", "number"],
    ["true", "boolean"],
    ["null", "null"],
  ])("rejects %s without echoing the input", (text, kind) => {
    expect(() => parseDescriptor(text)).toThrow(
      new Error(`Invalid descriptor: expected a JSON object, got ${kind}`),
    )
  })
})
