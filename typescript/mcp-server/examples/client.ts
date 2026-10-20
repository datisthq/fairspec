import { Client } from "@modelcontextprotocol/sdk/client/index.js"
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js"

const client = new Client({ name: "example", version: "0.0.0" })
await client.connect(
  new StdioClientTransport({ command: "node", args: ["mcp-server/main.ts"] }),
)

console.log(await client.listTools())
await client.close()
