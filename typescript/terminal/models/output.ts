import { z } from "zod"

// Item schemas stay `z.unknown()`: fireargs parses every output, and parsing each
// error or row of a large result through its full model costs seconds and a copy
export const ReportResult = z.object({
  valid: z.boolean().describe("Whether the validation passed without errors"),
  errors: z.array(z.unknown()).describe("Array of validation errors encountered"),
})

export const Records = z.array(z.unknown()).describe("Table rows, one object per row")

export const TextResult = z.object({ result: z.string() })

export const Empty = z.object({})
