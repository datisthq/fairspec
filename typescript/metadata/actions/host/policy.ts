import type { HostPolicy } from "../../models/host.ts"

export const HOST_POLICY_ENV = "FAIRSPEC_HOST_POLICY"

let hostPolicy: HostPolicy | undefined

/**
 * Return the policy remote fetches follow: the one set, then FAIRSPEC_HOST_POLICY, then "any"
 */
export function getHostPolicy() {
  if (hostPolicy) return hostPolicy

  const value = globalThis.process?.env?.[HOST_POLICY_ENV]
  if (!value) return "any"
  if (value === "any" || value === "public") return value

  throw new Error(`Invalid ${HOST_POLICY_ENV}: ${value} (expected "any" or "public")`)
}

/**
 * Set the policy remote fetches follow; undefined falls back to FAIRSPEC_HOST_POLICY
 */
export function setHostPolicy(policy: HostPolicy | undefined) {
  hostPolicy = policy
}
