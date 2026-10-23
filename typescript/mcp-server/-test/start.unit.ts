import { getHostPolicy, setHostPolicy } from "@fairspec/library"
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js"
import { afterEach, describe, expect, it, vi } from "vite-plus/test"
import { startServer } from "../server.ts"

describe("startServer", () => {
  afterEach(() => {
    setHostPolicy(undefined)
    vi.unstubAllEnvs()
    vi.restoreAllMocks()
  })

  it("blocks non-public hosts by default", async () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", undefined)
    const connect = vi.spyOn(McpServer.prototype, "connect").mockResolvedValue()

    await startServer()

    expect(connect).toHaveBeenCalledOnce()
    expect(getHostPolicy()).toBe("public")
  })

  it("keeps the host policy from the environment", async () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", "any")
    vi.spyOn(McpServer.prototype, "connect").mockResolvedValue()

    await startServer()

    expect(getHostPolicy()).toBe("any")
  })
})
