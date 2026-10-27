import { describe, expect, it } from "vite-plus/test"
import { DATA_SCHEMA_FORMATS } from "../format.ts"

describe("DATA_SCHEMA_FORMATS", () => {
  it.each([
    ["date", "2024-02-29", true],
    ["date", "2023-02-29", false],
    ["date", "2024-13-01", false],
    ["time", "23:59:60Z", true],
    ["time", "15:59:60-08:00", true],
    ["time", "12:00:60Z", false],
    ["time", "12:00:00", false],
    ["date-time", "2024-01-31t10:00:00.5+01:00", true],
    ["date-time", "2024-01-31 10:00:00Z", false],
    ["duration", "P1W", true],
    ["duration", "P", false],
    ["email", "joe@example.com", true],
    ["email", "joe", false],
    ["ipv4", "192.168.0.1", true],
    ["ipv4", "01.1.1.1", false],
    ["uuid", "123E4567-e89b-12d3-a456-426614174000", true],
    ["uuid", "123e4567e89b12d3a456426614174000", false],
  ])("%s %s is %s", (format, value, expected) => {
    expect(DATA_SCHEMA_FORMATS[format]?.(value)).toBe(expected)
  })
})
