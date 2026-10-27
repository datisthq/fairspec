import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { queryTable, queryTables, setNullsLast } from "../query.ts"

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

describe("setNullsLast", () => {
  it("should sort nulls last in descending order", async () => {
    const table = pl.DataFrame({ x: [1, null, 2] }).lazy()

    const frame = await queryTable(table, "SELECT x FROM self ORDER BY x DESC").collect()

    expect(frame.getColumn("x").toArray()).toEqual([2, 1, null])
  })

  it("should keep an explicit null order", () => {
    const query = "SELECT x FROM self ORDER BY x DESC NULLS FIRST, y"

    expect(setNullsLast(query)).toBe(
      "SELECT x FROM self ORDER BY x DESC NULLS FIRST, y NULLS LAST",
    )
  })

  it("should end items at parentheses and limits", () => {
    const query = "SELECT * FROM (SELECT x FROM self ORDER BY f(x) LIMIT 2) ORDER BY 1"

    expect(setNullsLast(query)).toBe(
      "SELECT * FROM (SELECT x FROM self ORDER BY f(x) NULLS LAST LIMIT 2) ORDER BY 1 NULLS LAST",
    )
  })

  it("should skip quoted text and comments", () => {
    const query = `SELECT 'order by' AS "ORDER BY" FROM self -- order by x\nORDER BY a`

    expect(setNullsLast(query)).toBe(
      `SELECT 'order by' AS "ORDER BY" FROM self -- order by x\nORDER BY a NULLS LAST`,
    )
  })
})
