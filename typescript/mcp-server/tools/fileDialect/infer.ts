import {
  FileDialect,
  InferFileDialectOptions,
  inferFileDialect,
  Resource,
} from "@fairspec/library"
import { z } from "zod"
import { createTool } from "../../helpers/tool.ts"

export const inferFileDialectTool = createTool({
  name: "inferFileDialectTool",
  description:
    "Infer the dialect (format and format-specific properties) from a resource. Analyzes the data to determine CSV delimiters, JSON structure, etc.",
  inputSchema: z.object({
    resource: Resource.describe("The resource to infer dialect from"),
    options: InferFileDialectOptions.optional().describe("Dialect inference options"),
  }),
  outputSchema: z.object({
    fileDialect: FileDialect.optional(),
  }),
  execute: async input => {
    const fileDialect = await inferFileDialect(input.resource, input.options)
    return { fileDialect }
  },
})
