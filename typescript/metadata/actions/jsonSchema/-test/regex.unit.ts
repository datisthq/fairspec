import { describe, expect, it } from "vite-plus/test"
import { isValidEcmaRegex } from "../regex.ts"

describe("isValidEcmaRegex", () => {
  it.each([
    "^[A-Z]{3}-[0-9]{4}$",
    "^(?<year>\\d{4})-\\k<year>$",
    "^\\p{L}+$",
    "\\u{1F600}",
    "[^]",
    "(?=a)b(?<!c)",
    "(a)\\1",
    "(?i:abc)",
    "[\\w.\\-]+\\@",
    "\\ud83d\\ude00",
    "(?<x>a)|(?<x>b)",
  ])("accepts %s", pattern => {
    expect(isValidEcmaRegex(pattern)).toBe(true)
  })

  it.each([
    "[a-",
    "(?i)abc",
    "(?P<n>x)",
    "a{,3}",
    "a{3,1}",
    "\\Z",
    "(a",
    "a)",
    "*a",
    "\\1",
    "\\k<y>",
    "(?<x>a)(?<x>b)",
    "[z-a]",
    "\\u{110000}",
  ])("rejects %s", pattern => {
    expect(isValidEcmaRegex(pattern)).toBe(false)
  })
})
