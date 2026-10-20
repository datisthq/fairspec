import { afterEach, describe, expect, it, vi } from "vite-plus/test"
import { CkanSchema } from "../../models/schema.ts"
import { makeCkanApiRequest } from "../ckan.ts"

describe("makeCkanApiRequest", () => {
  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("returns the result parsed by the schema", async () => {
    mockFetchJson({
      success: true,
      result: { fields: [{ id: "name", type: "text" }], records: [] },
    })

    const result = await makeCkanApiRequest({
      ckanUrl: "https://ckan.example.com/dataset/test",
      action: "datastore_search",
      payload: { resource_id: "1" },
      schema: CkanSchema,
    })

    expect(result).toEqual({ fields: [{ id: "name", type: "text" }] })
  })

  it("rejects a result that does not match the schema", async () => {
    mockFetchJson({ success: true, result: { fields: [{ id: "name" }] } })

    await expect(
      makeCkanApiRequest({
        ckanUrl: "https://ckan.example.com/dataset/test",
        action: "datastore_search",
        payload: { resource_id: "1" },
        schema: CkanSchema,
      }),
    ).rejects.toThrow(
      /CKAN API returned an unexpected response from datastore_search[\s\S]*fields\[0\]\.type/,
    )
  })

  it("reports the error of an unsuccessful response", async () => {
    mockFetchJson({
      success: false,
      error: { message: "Not found", __type: "Not Found Error" },
    })

    await expect(
      makeCkanApiRequest({
        ckanUrl: "https://ckan.example.com/dataset/test",
        action: "package_show",
        payload: { id: "test" },
        schema: CkanSchema,
      }),
    ).rejects.toThrow("CKAN API error: Not found")
  })
})

function mockFetchJson(body: unknown) {
  vi.spyOn(globalThis, "fetch").mockResolvedValue(new Response(JSON.stringify(body)))
}
