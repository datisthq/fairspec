import { z } from "zod"
import { DEFAULT_LIMIT, MAX_LIMIT } from "../../helpers/limit.ts"

export const MaxErrors = z
  .number()
  .int()
  .min(1)
  .max(MAX_LIMIT)
  .optional()
  .describe(`Maximum number of errors to return (default ${DEFAULT_LIMIT})`)

export const Truncated = z
  .boolean()
  .describe("Whether errors were left out of the report")
