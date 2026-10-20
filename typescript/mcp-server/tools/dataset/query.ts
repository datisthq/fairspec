import { Dataset, queryDataset } from "@fairspec/library"
import { z } from "zod"
import { DEFAULT_LIMIT, limitItems, MAX_LIMIT } from "../../helpers/limit.ts"
import { createTool } from "../../helpers/tool.ts"

export const queryDatasetTool = createTool({
  name: "queryDatasetTool",
  description:
    "Query a dataset using SQL. Each table resource is available under its name, so a query can join several resources. Returns the results as an array of records. Returns at most `limit` records; `truncated` is true when more were left out.",
  inputSchema: z.object({
    source: z
      .union([Dataset, z.string()])
      .describe("The dataset to query (dataset object or file path)"),
    query: z
      .string()
      .describe("SQL query to execute (use resource names as table names)"),
    limit: z
      .number()
      .int()
      .min(1)
      .max(MAX_LIMIT)
      .optional()
      .describe(`Maximum number of records to return (default ${DEFAULT_LIMIT})`),
  }),
  outputSchema: z.object({
    records: z.array(z.record(z.string(), z.unknown())),
    truncated: z.boolean().describe("Whether records were left out"),
  }),
  execute: async input => {
    const { limit = DEFAULT_LIMIT } = input

    const table = await queryDataset(input.source, input.query)
    const frame = await table.head(limit + 1).collect()

    const { items: records, truncated } = limitItems(frame.toRecords(), { limit })
    return { records, truncated }
  },
})
