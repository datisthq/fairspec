import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { queryTable, queryTables } from "../query.ts"

describe("queryTable", () => {
  it("should expose the table as self", async () => {
    const table = pl.DataFrame({ name: ["Alice", "Bob"], age: [30, 25] }).lazy()

    const frame = await queryTable(
      table,
      "SELECT name FROM self WHERE age > 26",
    ).collect()

    expect(frame.toRecords()).toEqual([{ name: "Alice" }])
  })

  it("should throw the SQL error of an invalid query", () => {
    const table = pl.DataFrame({ id: [1] }).lazy()

    expect(() => queryTable(table, "SELEC nonsense")).toThrow(/sql parser error/)
  })
})

describe("queryTables", () => {
  it("should join tables by name", async () => {
    const users = pl.DataFrame({ id: [1, 2], name: ["Alice", "Bob"] }).lazy()
    const posts = pl.DataFrame({ user_id: [2, 1, 2], title: ["a", "b", "c"] }).lazy()

    const frame = await queryTables(
      { users, posts },
      "SELECT name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id " +
        "GROUP BY name ORDER BY name",
    ).collect()

    expect(frame.toRecords()).toEqual([
      { name: "Alice", n: 1 },
      { name: "Bob", n: 2 },
    ])
  })
})
