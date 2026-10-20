import { DataSchema, inferDataSchema, Resource } from "@fairspec/library"
import { z } from "zod"
import { createTool } from "../../helpers/tool.ts"

export const inferDataSchemaTool = createTool({
  name: "inferDataSchemaTool",
  description:
    "Infer a data schema (JSON Schema) from data. Analyzes the data structure to determine types and properties.",
  inputSchema: z.object({
    resource: Resource.describe("The data resource to infer schema from"),
  }),
  outputSchema: z.object({
    dataSchema: DataSchema.optional(),
  }),
  execute: async input => {
    const dataSchema = await inferDataSchema(input.resource)
    return { dataSchema }
  },
})
