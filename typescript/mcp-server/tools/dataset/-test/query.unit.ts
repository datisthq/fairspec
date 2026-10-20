import type { Dataset } from "@fairspec/library"
import { describe, expect, it } from "vite-plus/test"
import { queryDatasetTool } from "../query.ts"

describe("queryDatasetTool", () => {
  it("validates tool structure", () => {
    expect(queryDatasetTool.name).toBe("queryDatasetTool")
    expect(queryDatasetTool.description).toBeTruthy()
    expect(queryDatasetTool.inputSchema).toBeTruthy()
    expect(queryDatasetTool.outputSchema).toBeTruthy()
    expect(queryDatasetTool.execute).toBeTypeOf("function")
  })

  it("joins two resources by name", async () => {
    const source: Dataset = {
      resources: [
        {
          name: "users",
          data: [
            { id: 1, name: "alice" },
            { id: 2, name: "bob" },
          ],
        },
        {
          name: "posts",
          data: [
            { id: 1, user_id: 2 },
            { id: 2, user_id: 2 },
          ],
        },
      ],
    }

    const result = await queryDatasetTool.execute({
      source,
      query:
        "SELECT users.name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id GROUP BY users.name",
    })

    expect(result.records).toEqual([{ name: "bob", n: 2 }])
  })

  it("returns at most limit records", async () => {
    const source: Dataset = {
      resources: [
        { name: "items", data: Array.from({ length: 10 }, (_, id) => ({ id })) },
      ],
    }

    const result = await queryDatasetTool.execute({
      source,
      query: "SELECT * FROM items",
      limit: 5,
    })

    expect(result.records).toHaveLength(5)
    expect(result.truncated).toBe(true)
  })
})
