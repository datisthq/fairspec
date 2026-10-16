import { LoadTableOptions } from "@fairspec/table"
import { z } from "zod"

export const ValidateTableOptions = LoadTableOptions.and(
  z.object({
    noInfer: z
      .boolean()
      .optional()
      .describe("Skip schema inference if tableSchema not provided"),
    maxErrors: z.number().optional().describe("Maximum number of errors to return"),
  }),
)

export type ValidateTableOptions = z.infer<typeof ValidateTableOptions>
