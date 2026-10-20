import { startServer } from "@fairspec/mcp-server"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"

export const mcpCommand = new Command()
  .name("mcp")
  .description("Start the MCP server")
  .configureHelp(helpConfiguration)

  .action(async () => {
    startServer().catch(error => {
      console.error("Error running MCP server:", error)
      process.exit(1)
    })
  })
