import { Descriptor, Report, validateDataSchema } from "@fairspec/library"
import { z } from "zod"
import { limitReport } from "../../helpers/limit.ts"
import { createTool } from "../../helpers/tool.ts"
import { MaxErrors, Truncated } from "../-shared/schemas.ts"

export const validateDataSchemaTool = createTool({
  name: "validateDataSchemaTool",
  description:
    "Validate a data schema (JSON Schema) against the Fairspec Data Schema specification. Returns validation report with errors if any.",
  inputSchema: z.object({
    source: z
      .union([Descriptor, z.string()])
      .describe("The data schema to validate (descriptor object or file path)"),
    maxErrors: MaxErrors,
  }),
  outputSchema: z.object({
    report: Report,
    truncated: Truncated,
  }),
  execute: async input => {
    const report = await validateDataSchema(input.source)
    return limitReport(report, { maxErrors: input.maxErrors })
  },
})
