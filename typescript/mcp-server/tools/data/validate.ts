import { Report, Resource, validateData } from "@fairspec/library"
import { z } from "zod"
import { limitReport } from "../../helpers/limit.ts"
import { createTool } from "../../helpers/tool.ts"
import { MaxErrors, Truncated } from "../-shared/schemas.ts"

export const validateDataTool = createTool({
  name: "validateDataTool",
  description:
    "Validate data against a data schema (JSON Schema). Returns a validation report with errors if any.",
  inputSchema: z.object({
    resource: Resource.describe("The data resource to validate"),
    maxErrors: MaxErrors,
  }),
  outputSchema: z.object({
    report: Report,
    truncated: Truncated,
  }),
  execute: async input => {
    const report = await validateData(input.resource)
    return limitReport(report, { maxErrors: input.maxErrors })
  },
})
