import { loadTable, queryTable, Resource } from "@fairspec/library"
import { z } from "zod"
import { DEFAULT_LIMIT, limitItems, MAX_LIMIT } from "../../helpers/limit.ts"
import { createTool } from "../../helpers/tool.ts"

export const queryTableTool = createTool({
  name: "queryTableTool",
  description:
    "Query a table using SQL. Loads the table, executes the SQL query, and returns the results as an array of records. Returns at most `limit` records; `truncated` is true when more were left out.",
  inputSchema: z.object({
    resource: Resource.describe("The table resource to query"),
    query: z.string().describe("SQL query to execute (use 'self' as table name)"),
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

    const table = await loadTable(input.resource, {})
    if (!table) {
      throw new Error("Could not load table")
    }

    const frame = await queryTable(table, input.query)
      .head(limit + 1)
      .collect()

    const { items: records, truncated } = limitItems(frame.toRecords(), { limit })
    return { records, truncated }
  },
})
