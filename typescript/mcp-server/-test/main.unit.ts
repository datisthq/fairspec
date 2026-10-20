import { resolve } from "node:path"
import { writeTempFile } from "@fairspec/library"
import { Client } from "@modelcontextprotocol/sdk/client/index.js"
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js"
import { afterEach, beforeEach, describe, expect, it } from "vite-plus/test"

describe("main", () => {
  let client: Client
  let errors: Error[]

  beforeEach(async () => {
    errors = []
    client = new Client({ name: "test", version: "0.0.0" })
    client.onerror = error => errors.push(error)
    await client.connect(
      new StdioClientTransport({
        command: process.execPath,
        args: [resolve(import.meta.dirname, "../main.ts")],
        stderr: "pipe",
      }),
    )
  })

  afterEach(async () => {
    await client.close()
  })

  it("should serve the tools over stdio", async () => {
    const { tools } = await client.listTools()

    expect(tools.map(tool => tool.name)).toContain("inferTableSchemaTool")
  })

  it("should keep stdout to JSON-RPC messages during a tool call", async () => {
    const path = await writeTempFile("id,name\n1,alice\n2,bob", { format: "csv" })

    const result = await client.callTool({
      name: "inferTableSchemaTool",
      arguments: { resource: { data: path } },
    })

    expect(result.isError).toBeFalsy()
    expect(errors).toEqual([])
  })

  it("should report the real message of a failed tool over stdio", async () => {
    const result = await client.callTool({
      name: "validateDatasetTool",
      arguments: { source: "missing/dataset.json" },
    })

    expect(result.isError).toBe(true)
    expect(JSON.stringify(result.content)).toMatch(/missing/)
  })
})
