import { writeTempFile } from "@fairspec/dataset"
import type { Resource } from "@fairspec/metadata"
import { describe, expect, it } from "vite-plus/test"
import { loadData } from "../load.ts"

describe("loadData", () => {
  it("should return inline data when present", async () => {
    const resource: Resource = {
      data: { key: "value", nested: { foo: "bar" } },
    }

    const data = await loadData(resource)
    expect(data).toEqual({ key: "value", nested: { foo: "bar" } })
  })

  it("should load data from JSON file path", async () => {
    const text = JSON.stringify({ id: 1, name: "test" })
    const path = await writeTempFile(text, { format: "json" })

    const resource: Resource = { data: path }

    const data = await loadData(resource)
    expect(data).toEqual({ id: 1, name: "test" })
  })

  it("should load data from JSON format", async () => {
    const text = JSON.stringify({ id: 1, name: "test" })
    const path = await writeTempFile(text)

    const resource: Resource = { data: path, fileDialect: { format: "json" } }

    const data = await loadData(resource)
    expect(data).toEqual({ id: 1, name: "test" })
  })

  it("should load a top-level array from a JSON file", async () => {
    const path = await writeTempFile("[1, 2, 3]", { format: "json" })

    const data = await loadData({ data: path })
    expect(data).toEqual([1, 2, 3])
  })

  it("should load an empty object from a JSON file", async () => {
    const path = await writeTempFile("{}", { format: "json" })

    const data = await loadData({ data: path })
    expect(data).toEqual({})
  })

  it("should return undefined when no data available", async () => {
    const resource: Resource = { name: "empty-resource" }

    const data = await loadData(resource)
    expect(data).toBeUndefined()
  })
})
