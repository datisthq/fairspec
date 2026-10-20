import {
  InferTableSchemaOptions,
  inferTableSchema,
  Resource,
  TableSchema,
} from "@fairspec/library"
import { z } from "zod"
import { createTool } from "../../helpers/tool.ts"

export const inferTableSchemaTool = createTool({
  name: "inferTableSchemaTool",
  description:
    "Infer a table schema from table resource data. Analyzes the data to determine column types and constraints.",
  inputSchema: z.object({
    resource: Resource.describe("The table resource to infer schema from"),
    options: InferTableSchemaOptions.optional().describe("Schema inference options"),
  }),
  outputSchema: z.object({
    tableSchema: TableSchema.optional(),
  }),
  execute: async input => {
    const tableSchema = await inferTableSchema(input.resource, input.options)
    return { tableSchema }
  },
})
