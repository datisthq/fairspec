import { afterEach, describe, expect, it, vi } from "vite-plus/test"
import { ZenodoDeposition } from "../../models/deposition.ts"
import { makeZenodoApiRequest } from "../zenodo.ts"

describe("makeZenodoApiRequest", () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("returns the response parsed by the schema", async () => {
    mockFetchJson({
      id: 1,
      links: { html: "https://zenodo.org/deposit/1" },
      unknown: "dropped",
    })

    const deposition = await makeZenodoApiRequest({
      endpoint: "/deposit/depositions/1",
      schema: ZenodoDeposition,
    })

    expect(deposition).toEqual({ id: 1, links: { html: "https://zenodo.org/deposit/1" } })
  })

  it("rejects a response that does not match the schema", async () => {
    mockFetchJson({ id: "1", links: {} })

    await expect(
      makeZenodoApiRequest({
        endpoint: "/deposit/depositions/1",
        schema: ZenodoDeposition,
      }),
    ).rejects.toThrow(
      /Zenodo API returned an unexpected response from \/deposit\/depositions\/1[\s\S]*links\.html/,
    )
  })
})

function mockFetchJson(body: unknown) {
  vi.spyOn(globalThis, "fetch").mockResolvedValue(new Response(JSON.stringify(body)))
}
