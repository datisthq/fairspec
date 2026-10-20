import {
  inferTableSchema,
  Report,
  Resource,
  resolveTableSchema,
  ValidateTableOptions,
  validateTable,
} from "@fairspec/library"
import { z } from "zod"
import { DEFAULT_LIMIT, limitReport, MAX_LIMIT } from "../../helpers/limit.ts"
import { createTool } from "../../helpers/tool.ts"
import { MaxErrors, Truncated } from "../-shared/schemas.ts"

export const validateTableTool = createTool({
  name: "validateTableTool",
  description:
    "Validate a table resource against its schema. Returns a validation report with errors if any.",
  inputSchema: z.object({
    resource: Resource.describe("The table resource to validate"),
    options: ValidateTableOptions.optional().describe("Validation options"),
    maxErrors: MaxErrors,
  }),
  outputSchema: z.object({
    report: Report,
    truncated: Truncated,
  }),
  execute: async input => {
    const { resource, options } = input
    const maxErrors = Math.min(
      input.maxErrors ?? options?.maxErrors ?? DEFAULT_LIMIT,
      MAX_LIMIT,
    )

    const tableSchema =
      (await resolveTableSchema(resource.tableSchema)) ??
      (options?.noInfer ? undefined : await inferTableSchema(resource, options))

    // The library splits maxErrors across columns, so ask every column for one error
    // more than the limit, or a single bad column would never show truncation
    const columnCount = Math.max(Object.keys(tableSchema?.properties ?? {}).length, 1)
    const report = await validateTable(
      { ...resource, tableSchema },
      { ...options, maxErrors: (maxErrors + 1) * columnCount },
    )

    return limitReport(report, { maxErrors })
  },
})
