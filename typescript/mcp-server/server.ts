import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js"
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js"
import packageJson from "./package.json" with { type: "json" }
import { validateDataTool } from "./tools/data/validate.ts"
import { inferDataSchemaTool } from "./tools/dataSchema/infer.ts"
import { validateDataSchemaTool } from "./tools/dataSchema/validate.ts"
import { inferDatasetTool } from "./tools/dataset/infer.ts"
import { queryDatasetTool } from "./tools/dataset/query.ts"
import { validateDatasetTool } from "./tools/dataset/validate.ts"
import { previewFileTool } from "./tools/file/preview.ts"
import { inferFileDialectTool } from "./tools/fileDialect/infer.ts"
import { queryTableTool } from "./tools/table/query.ts"
import { validateTableTool } from "./tools/table/validate.ts"
import { inferTableSchemaTool } from "./tools/tableSchema/infer.ts"
import { validateTableSchemaTool } from "./tools/tableSchema/validate.ts"

const tools = [
  inferDataSchemaTool,
  inferDatasetTool,
  inferFileDialectTool,
  inferTableSchemaTool,
  previewFileTool,
  queryDatasetTool,
  queryTableTool,
  validateDataSchemaTool,
  validateDatasetTool,
  validateDataTool,
  validateTableSchemaTool,
  validateTableTool,
]

/**
 * Create the Fairspec MCP server with every tool registered.
 */
export function createServer() {
  const server = new McpServer({ name: "fairspec", version: packageJson.version })

  for (const tool of tools) {
    server.registerTool(
      tool.name,
      {
        description: tool.description,
        inputSchema: tool.inputSchema,
        outputSchema: tool.outputSchema,
      },
      tool.handle,
    )
  }

  return server
}

/**
 * Serve the Fairspec MCP server over stdio.
 */
export async function startServer() {
  await createServer().connect(new StdioServerTransport())
}
