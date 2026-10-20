import type { Dataset } from "@fairspec/library"
import { describe, expect, it } from "vite-plus/test"
import { queryDatasetTool } from "../query.ts"

describe("queryDatasetTool", () => {
  it("validates tool structure", () => {
    expect(queryDatasetTool.id).toBe("query-dataset")
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

    const result = await queryDatasetTool.execute?.(
      {
        source,
        query:
          "SELECT users.name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id GROUP BY users.name",
      },
      {},
    )

    expect.assert(result)
    expect.assert(!("error" in result))
    expect(result.records).toEqual([{ name: "bob", n: 2 }])
  })
})
