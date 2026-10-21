import { execFile } from "node:child_process"
import { resolve } from "node:path"
import { promisify } from "node:util"
import { writeTempFile } from "@fairspec/library"
import { Client } from "@modelcontextprotocol/sdk/client/index.js"
import { StdioClientTransport } from "@modelcontextprotocol/sdk/client/stdio.js"
import { describe, expect, it } from "vite-plus/test"
import metadata from "../../package.json" with { type: "json" }

// The published bin is the bundle, which the source-level tests never load
const BIN = resolve(import.meta.dirname, "../../build/entrypoints/run.js")

describe("run", () => {
  it("should print the package version", async () => {
    const { stdout } = await runBin("--version")

    expect(stdout.trim()).toBe(metadata.version)
  })

  it("should validate a table", async () => {
    const data = await writeTempFile("id,name\n1,alice\n2,bob", { format: "csv" })
    const schema = await writeTempFile(
      JSON.stringify({
        properties: { id: { type: "integer" }, name: { type: "string" } },
      }),
      { format: "json" },
    )

    const { stdout } = await runBin(
      "table",
      "validate",
      data,
      "--schema",
      schema,
      "--json",
    )

    expect(JSON.parse(stdout)).toEqual({ valid: true, errors: [] })
  })

  it("should report an invalid table", async () => {
    const data = await writeTempFile("id\n1\nbad", { format: "csv" })
    const schema = await writeTempFile(
      JSON.stringify({ properties: { id: { type: "integer" } } }),
      { format: "json" },
    )

    const result = await runBin("table", "validate", data, "--schema", schema, "--json")

    expect(JSON.parse(result.stdout)).toMatchObject({
      valid: false,
      errors: [{ type: "cell/type", rowNumber: 2 }],
    })
  })

  it("should serve the MCP tools over stdio", async () => {
    const client = new Client({ name: "test", version: "0.0.0" })
    await client.connect(
      new StdioClientTransport({
        command: process.execPath,
        args: [BIN, "mcp"],
        stderr: "pipe",
      }),
    )

    try {
      const { tools } = await client.listTools()
      expect(tools.map(tool => tool.name)).toContain("validateTableTool")
    } finally {
      await client.close()
    }
  })
})

async function runBin(...args: string[]) {
  try {
    return await promisify(execFile)(process.execPath, [BIN, ...args])
  } catch (error) {
    // An invalid table exits 1 with its report still on stdout
    if (error instanceof Error && "stdout" in error && typeof error.stdout === "string") {
      return { stdout: error.stdout }
    }
    throw error
  }
}
