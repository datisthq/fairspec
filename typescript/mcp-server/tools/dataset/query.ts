import { Dataset, queryDataset } from "@fairspec/library"
import { createTool } from "@mastra/core/tools"
import { z } from "zod"

export const queryDatasetTool = createTool({
  id: "query-dataset",
  description:
    "Query a dataset using SQL. Each table resource is available under its name, so a query can join several resources. Returns the results as an array of records.",
  inputSchema: z.object({
    source: z
      .union([Dataset, z.string()])
      .describe("The dataset to query (dataset object or file path)"),
    query: z
      .string()
      .describe("SQL query to execute (use resource names as table names)"),
  }),
  outputSchema: z.object({
    records: z.array(z.record(z.string(), z.unknown())),
  }),
  execute: async input => {
    const table = await queryDataset(input.source, input.query)
    const frame = await table.collect()
    const records = frame.toRecords()
    return { records }
  },
})
