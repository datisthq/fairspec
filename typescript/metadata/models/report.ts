import { z } from "zod"
import { FairspecError } from "./error/error.ts"

export const Report = z.object({
  valid: z.boolean().describe("Whether the validation passed without errors"),

  errors: z.array(FairspecError).describe("Array of validation errors encountered"),
})

export type Report = z.infer<typeof Report>
