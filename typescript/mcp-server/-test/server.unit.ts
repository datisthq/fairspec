import { writeTempFile } from "@fairspec/library"
import { Client } from "@modelcontextprotocol/sdk/client/index.js"
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { createServer } from "../server.ts"

describe("createServer", () => {
  let client: Client

  beforeEach(async () => {
    const [clientTransport, serverTransport] = InMemoryTransport.createLinkedPair()
    await createServer().connect(serverTransport)
    client = new Client({ name: "test", version: "0.0.0" })
    await client.connect(clientTransport)
  })

  afterEach(async () => {
    await client.close()
  })

  it("lists every tool under its published name", async () => {
    const { tools } = await client.listTools()

    expect(tools.map(tool => tool.name).sort()).toEqual([
      "inferDataSchemaTool",
      "inferDatasetTool",
      "inferFileDialectTool",
      "inferTableSchemaTool",
      "queryDatasetTool",
      "queryTableTool",
      "validateDataSchemaTool",
      "validateDataTool",
      "validateDatasetTool",
      "validateTableSchemaTool",
      "validateTableTool",
    ])
  })

  it("returns structured content with a text copy", async () => {
    const path = await writeTempFile("id,name\n1,alice\n2,bob", { format: "csv" })

    const result = await client.callTool({
      name: "queryTableTool",
      arguments: { resource: { data: path }, query: "SELECT * FROM self" },
    })

    expect(result.isError).toBeFalsy()
    expect(result.structuredContent).toEqual({
      records: [
        { id: 1, name: "alice" },
        { id: 2, name: "bob" },
      ],
      truncated: false,
    })
  })

  it("reports the real message of a failed query", async () => {
    const path = await writeTempFile("id,name\n1,alice", { format: "csv" })

    const result = await client.callTool({
      name: "queryTableTool",
      arguments: { resource: { data: path }, query: "SELEC nonsense" },
    })

    expect(result.isError).toBe(true)
    expect(JSON.stringify(result.content)).toMatch(/sql/i)
    expect(JSON.stringify(result.content)).not.toMatch(/ZodError|structured content/)
  })

  it("reports the real message of a missing file", async () => {
    const result = await client.callTool({
      name: "validateDatasetTool",
      arguments: { source: "missing/dataset.json" },
    })

    expect(result.isError).toBe(true)
    expect(JSON.stringify(result.content)).toMatch(/missing/)
    expect(JSON.stringify(result.content)).not.toMatch(/ZodError|structured content/)
  })

  it("rejects a response over the budget", async () => {
    const result = await client.callTool({
      name: "inferDatasetTool",
      arguments: {
        dataset: { resources: [{ data: [{ text: "x".repeat(60_000) }] }] },
      },
    })

    expect(result.isError).toBe(true)
    expect(JSON.stringify(result.content)).toMatch(/exceeds 50000 characters/)
  })

  it("writes nothing to stdout", async () => {
    const write = vi.spyOn(process.stdout, "write")

    await client.callTool({
      name: "inferTableSchemaTool",
      arguments: { resource: { data: [{ id: 1 }] } },
    })

    expect(write).not.toHaveBeenCalled()
    write.mockRestore()
  })
})
