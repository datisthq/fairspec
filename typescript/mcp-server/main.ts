import { startServer } from "./server.ts"

startServer().catch(error => {
  console.error("Error running MCP server:", error)
  process.exit(1)
})
