import { createServer } from "node:http"
import { afterAll, afterEach, beforeAll, describe, expect, it, vi } from "vite-plus/test"
import { setHostPolicy } from "../../host/policy.ts"
import { fetchRemote } from "../fetch.ts"

vi.mock("../../host/address.ts", async importOriginal => {
  const actual = await importOriginal<typeof import("../../host/address.ts")>()
  return {
    ...actual,
    assertPublicAddress: (host: string, address: string) => {
      if (allowLoopbackLiteral && host === "127.0.0.1") return
      actual.assertPublicAddress(host, address)
    },
  }
})

let allowLoopbackLiteral = false

describe("fetchRemote", () => {
  const server = createServer((request, response) => {
    if (request.url === "/data.json") {
      response.end('{"name":"test"}')
    } else if (request.url === "/redirect") {
      response.writeHead(302, { location: "http://169.254.169.254/latest/meta-data/" })
      response.end()
    } else if (request.url === "/redirect-name") {
      response.writeHead(302, { location: `http://localhost:${getPort()}/data.json` })
      response.end()
    } else {
      response.writeHead(404)
      response.end()
    }
  })

  const getPort = () => {
    const address = server.address()
    return typeof address === "object" && address ? address.port : 0
  }
  const getUrl = (path: string) => `http://127.0.0.1:${getPort()}${path}`

  beforeAll(async () => {
    await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve))
  })

  afterAll(async () => {
    await new Promise(resolve => server.close(resolve))
  })

  afterEach(() => {
    setHostPolicy(undefined)
    allowLoopbackLiteral = false
    vi.unstubAllEnvs()
    vi.unstubAllGlobals()
  })

  it("reaches the loopback host under the any policy", async () => {
    const response = await fetchRemote(getUrl("/data.json"))
    expect(await response.text()).toBe('{"name":"test"}')
  })

  it("blocks a loopback literal under the public policy", async () => {
    setHostPolicy("public")
    await expect(fetchRemote(getUrl("/data.json"))).rejects.toThrow(
      "Host 127.0.0.1 resolves to non-public address 127.0.0.1",
    )
  })

  it("blocks a name resolving to loopback under the public policy", async () => {
    setHostPolicy("public")
    await expect(fetchRemote(`http://localhost:${getPort()}/data.json`)).rejects.toThrow(
      /Host localhost resolves to non-public address/,
    )
  })

  it("reads the public policy from the environment", async () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", "public")
    await expect(fetchRemote(getUrl("/data.json"))).rejects.toThrow("non-public address")
  })

  it("re-checks a redirect to a literal address", async () => {
    setHostPolicy("public")
    allowLoopbackLiteral = true

    const response = await fetchRemote(getUrl("/data.json"))
    expect(await response.text()).toBe('{"name":"test"}')
    await expect(fetchRemote(getUrl("/redirect"))).rejects.toThrow(
      "Host 169.254.169.254 resolves to non-public address 169.254.169.254",
    )
  })

  it("re-checks a redirect to a resolved name", async () => {
    setHostPolicy("public")
    allowLoopbackLiteral = true

    await expect(fetchRemote(getUrl("/redirect-name"))).rejects.toThrow(
      /Host localhost resolves to non-public address/,
    )
  })

  it("returns an error status under the public policy", async () => {
    setHostPolicy("public")
    allowLoopbackLiteral = true

    const response = await fetchRemote(getUrl("/missing"))
    expect(response.status).toBe(404)
  })

  it("rejects other protocols under the public policy", async () => {
    setHostPolicy("public")
    await expect(fetchRemote("ftp://example.com/file.csv")).rejects.toThrow(
      "Unsupported remote protocol: ftp",
    )
  })

  it("refuses host names outside Node.js under the public policy", async () => {
    setHostPolicy("public")
    vi.stubGlobal("Deno", {})

    await expect(fetchRemote("http://example.com/data.json")).rejects.toThrow(
      "Cannot check host example.com under the public host policy outside Node.js",
    )
    await expect(fetchRemote("http://10.0.0.1/data.json")).rejects.toThrow(
      "Host 10.0.0.1 resolves to non-public address 10.0.0.1",
    )
  })

  it("refuses redirects outside Node.js under the public policy", async () => {
    setHostPolicy("public")
    vi.stubGlobal("Deno", {})
    const fetchMock = vi.fn().mockResolvedValue(new Response("ok"))
    vi.stubGlobal("fetch", fetchMock)

    await fetchRemote("http://8.8.8.8/data.json")
    expect(fetchMock).toHaveBeenCalledWith("http://8.8.8.8/data.json", {
      redirect: "error",
    })
  })
})
