import type { Dataset } from "@fairspec/metadata"
import { describe, expect, it } from "vite-plus/test"
import { queryDataset } from "../query.ts"

const users = [
  { id: 1, name: "Alice" },
  { id: 2, name: "Bob" },
]

const posts = [
  { id: 1, user_id: 2 },
  { id: 2, user_id: 1 },
  { id: 3, user_id: 2 },
]

describe("queryDataset", () => {
  it("should join resources by name", async () => {
    const dataset: Dataset = {
      resources: [
        { name: "users", data: users },
        { name: "posts", data: posts },
      ],
    }

    const table = await queryDataset(
      dataset,
      "SELECT name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id " +
        "GROUP BY name ORDER BY name",
    )

    expect((await table.collect()).toRecords()).toEqual([
      { name: "Alice", n: 1 },
      { name: "Bob", n: 2 },
    ])
  })

  it("should expose an unnamed resource under its inferred name", async () => {
    const dataset: Dataset = { resources: [{ data: users }] }

    const table = await queryDataset(dataset, "SELECT COUNT(*) AS n FROM resource1")

    expect((await table.collect()).toRecords()).toEqual([{ n: 2 }])
  })

  it("should not load a resource the query does not name", async () => {
    const dataset: Dataset = {
      resources: [
        { name: "users", data: users },
        { name: "broken", data: "missing.csv" },
      ],
    }

    const table = await queryDataset(dataset, "SELECT COUNT(*) AS n FROM users")

    expect((await table.collect()).toRecords()).toEqual([{ n: 2 }])
  })

  it("should skip a named resource that is not a table", async () => {
    const dataset: Dataset = {
      resources: [
        { name: "users", data: users },
        { name: "config", data: { key: "value" } },
      ],
    }

    const table = await queryDataset(
      dataset,
      "SELECT name FROM users WHERE name <> 'config' ORDER BY name",
    )

    expect((await table.collect()).toRecords()).toEqual([
      { name: "Alice" },
      { name: "Bob" },
    ])
  })

  it("should throw when two resources share a named table", async () => {
    const dataset: Dataset = {
      resources: [
        { name: "users", data: users },
        { name: "users", data: users },
      ],
    }

    await expect(queryDataset(dataset, "SELECT * FROM users")).rejects.toThrow(
      'More than one resource is named "users"',
    )
  })
})
