import type { LookupAddress, LookupOptions } from "node:dns"
import { HostPolicyError, assertPublicAddress } from "../host/address.ts"
import { getHostPolicy } from "../host/policy.ts"

const MAX_REDIRECTS = 10

/**
 * Fetch a remote URL under the host policy (see `setHostPolicy`)
 */
export async function fetchRemote(url: string, init?: RequestInit) {
  if (getHostPolicy() === "any") {
    return await fetch(url, init)
  }

  return await fetchPublicRemote(url, init)
}

// Node skips the dispatcher's lookup for IP literals, so each hop's literal host is checked
// here and redirects are followed by hand
async function fetchPublicRemote(url: string, init?: RequestInit) {
  // Bun and Deno report a Node version but may ignore the dispatcher, and a browser cannot
  // resolve names or show a redirect's target, so elsewhere only a literal public address
  // passes, without redirects
  if (!getIsNode()) {
    const target = new URL(url)
    assertPublicUrl(target)
    const host = target.hostname.replace(/^\[(.*)\]$/, "$1")
    if (!getIsAddressLiteral(host)) {
      throw new Error(
        `Cannot check host ${host} under the public host policy outside Node.js`,
      )
    }
    return await fetch(url, { ...init, redirect: "error" })
  }

  const { lookup } = await import("node:dns")
  const { Agent } = await import("undici")
  const dispatcher = new Agent({
    connect: { lookup: createPublicLookup(lookup) },
  })

  let target = new URL(url)
  let options: RequestInit = { ...init }
  for (let hop = 0; hop <= MAX_REDIRECTS; hop++) {
    assertPublicUrl(target)

    // Node's fetch takes the npm Agent at runtime, while @types/node pins older undici types
    const request: RequestInit = { ...options, redirect: "manual" }
    Object.assign(request, { dispatcher })

    const response = await fetch(target, request).catch(error => {
      throw error instanceof TypeError && error.cause instanceof HostPolicyError
        ? error.cause
        : error
    })

    const location = response.headers.get("location")
    if (response.status < 300 || response.status > 399 || !location) {
      return response
    }

    if (
      response.status === 303 ||
      ((response.status === 301 || response.status === 302) && options.method === "POST")
    ) {
      options = { ...options, method: "GET", body: undefined }
    }

    const next = new URL(location, target)
    if (next.origin !== target.origin) {
      const headers = new Headers(options.headers)
      headers.delete("authorization")
      headers.delete("cookie")
      options = { ...options, headers }
    }
    target = next
  }

  throw new Error(`Too many redirects: ${url}`)
}

function assertPublicUrl(url: URL) {
  const protocol = url.protocol.replace(/:$/, "")
  if (protocol !== "http" && protocol !== "https") {
    throw new Error(`Unsupported remote protocol: ${protocol}`)
  }

  const host = url.hostname.replace(/^\[(.*)\]$/, "$1")
  if (getIsAddressLiteral(host)) {
    assertPublicAddress(host, host)
  }
}

function getIsAddressLiteral(host: string) {
  return /^[\d.]+$/.test(host) || host.includes(":")
}

function getIsNode() {
  const versions: Record<string, string | undefined> | undefined =
    globalThis.process?.versions
  return !!versions?.node && !versions.bun && !("Deno" in globalThis)
}

function createPublicLookup(lookup: typeof import("node:dns").lookup) {
  return (
    hostname: string,
    options: LookupOptions,
    callback: (
      error: Error | null,
      address: string | LookupAddress[],
      family?: number,
    ) => void,
  ) => {
    lookup(hostname, { ...options, all: true }, (error, addresses) => {
      if (error) return callback(error, [])

      try {
        for (const { address } of addresses) assertPublicAddress(hostname, address)
      } catch (hostError) {
        return callback(hostError instanceof Error ? hostError : null, [])
      }

      const [first] = addresses
      if (options.all || !first) return callback(null, addresses)
      callback(null, first.address, first.family)
    })
  }
}
