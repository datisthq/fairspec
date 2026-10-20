import type { TableCheck as TableCheckType } from "@fairspec/table"
import { LoadTableOptions, TableCheck } from "@fairspec/table"
import { z } from "zod"

export const ValidateTableOptions = LoadTableOptions.and(
  z.object({
    noInfer: z
      .boolean()
      .optional()
      .describe("Skip schema inference if tableSchema not provided"),
    maxErrors: z.number().optional().describe("Maximum number of errors to return"),
    checks: z
      .array(TableCheck)
      .optional()
      .describe("Conditions every row, or the whole table, must satisfy"),
  }),
)

export type ValidateTableOptions = Omit<
  z.infer<typeof ValidateTableOptions>,
  "checks"
> & {
  checks?: TableCheckType[]
}
