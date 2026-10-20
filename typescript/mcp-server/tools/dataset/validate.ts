import { Dataset, Report, validateDataset } from "@fairspec/library"
import { z } from "zod"
import { limitReport } from "../../helpers/limit.ts"
import { createTool } from "../../helpers/tool.ts"
import { MaxErrors, Truncated } from "../-shared/schemas.ts"

export const validateDatasetTool = createTool({
  name: "validateDatasetTool",
  description:
    "Validate a dataset including all its resources and foreign keys. Returns a validation report with errors if any.",
  inputSchema: z.object({
    source: z
      .union([Dataset, z.string()])
      .describe("The dataset to validate (dataset object or file path)"),
    maxErrors: MaxErrors,
  }),
  outputSchema: z.object({
    report: Report,
    truncated: Truncated,
  }),
  execute: async input => {
    const report = await validateDataset(input.source)
    return limitReport(report, { maxErrors: input.maxErrors })
  },
})
