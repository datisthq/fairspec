import { startServer } from "@fairspec/mcp-server"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { Empty } from "../../models/output.ts"

export const mcpCommand = f
  .command({
    description: "Start the MCP server",
    configureHelp: helpConfiguration,
    jsonOption: false,
  })
  .input(z.object({}))
  .output(Empty)
  .handler(() => {
    startServer().catch(error => {
      console.error("Error running MCP server:", error)
      process.exit(1)
    })

    return {}
  })
