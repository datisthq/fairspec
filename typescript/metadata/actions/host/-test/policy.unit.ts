import { afterEach, describe, expect, it, vi } from "vite-plus/test"
import { getHostPolicy, setHostPolicy } from "../policy.ts"

describe("getHostPolicy", () => {
  afterEach(() => {
    setHostPolicy(undefined)
    vi.unstubAllEnvs()
  })

  it("defaults to any", () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", undefined)
    expect(getHostPolicy()).toBe("any")
  })

  it("reads the environment", () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", "public")
    expect(getHostPolicy()).toBe("public")
  })

  it("treats an empty environment value as unset", () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", "")
    expect(getHostPolicy()).toBe("any")
  })

  it("rejects an invalid environment value", () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", "private")
    expect(() => getHostPolicy()).toThrow("Invalid FAIRSPEC_HOST_POLICY: private")
  })

  it("prefers the policy set over the environment", () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", "any")
    setHostPolicy("public")
    expect(getHostPolicy()).toBe("public")
  })

  it("falls back to the environment once unset", () => {
    vi.stubEnv("FAIRSPEC_HOST_POLICY", undefined)
    setHostPolicy("public")
    setHostPolicy(undefined)
    expect(getHostPolicy()).toBe("any")
  })
})
