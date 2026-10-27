import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { inspectTable } from "../../inspect.ts"

describe("inspectTable (row/unique)", () => {
  it("should not errors when all rows are unique for primary key", async () => {
    const table = pl
      .DataFrame({
        id: [1, 2, 3, 4, 5],
        name: ["Alice", "Bob", "Charlie", "David", "Eve"],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: "number" },
        name: { type: "string" },
      },
      primaryKey: ["id"],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toHaveLength(0)
  })

  it("should errors for duplicate primary key rows", async () => {
    const table = pl
      .DataFrame({
        id: [1, 2, 3, 2, 5],
        name: ["Alice", "Bob", "Charlie", "Bob2", "Eve"],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: "number" },
        name: { type: "string" },
      },
      primaryKey: ["id"],
    }

    const errors = await inspectTable(table, { tableSchema })

    expect(errors.filter(e => e.type === "row/primaryKey")).toHaveLength(1)
    expect(errors).toEqual([
      {
        type: "row/primaryKey",
        rowNumber: 4,
        columnNames: ["id"],
      },
    ])
  })

  it("should not errors when all rows are unique for unique key", async () => {
    const table = pl
      .DataFrame({
        id: [1, 2, 3, 4, 5],
        email: ["a@test.com", "b@test.com", "c@test.com", "d@test.com", "e@test.com"],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: "number" },
        email: { type: "string" },
      },
      uniqueKeys: [["email"]],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toHaveLength(0)
  })

  it("should errors for duplicate unique key rows", async () => {
    const table = pl
      .DataFrame({
        id: [1, 2, 3, 4, 5],
        email: ["a@test.com", "b@test.com", "a@test.com", "d@test.com", "b@test.com"],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: "number" },
        email: { type: "string" },
      },
      uniqueKeys: [["email"]],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors.filter(e => e.type === "row/uniqueKey")).toHaveLength(2)
    expect(errors).toEqual([
      {
        type: "row/uniqueKey",
        rowNumber: 3,
        columnNames: ["email"],
      },
      {
        type: "row/uniqueKey",
        rowNumber: 5,
        columnNames: ["email"],
      },
    ])
  })

  it("should handle composite unique keys", async () => {
    const table = pl
      .DataFrame({
        category: ["A", "A", "B", "A", "B"],
        subcategory: ["X", "Y", "X", "X", "Y"],
        value: [1, 2, 3, 4, 5],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        category: { type: "string" },
        subcategory: { type: "string" },
        value: { type: "number" },
      },
      uniqueKeys: [["category", "subcategory"]],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors.filter(e => e.type === "row/uniqueKey")).toHaveLength(1)
    expect(errors).toEqual([
      {
        type: "row/uniqueKey",
        rowNumber: 4,
        columnNames: ["category", "subcategory"],
      },
    ])
  })

  it("should handle both primary key and unique keys", async () => {
    const table = pl
      .DataFrame({
        id: [1, 2, 3, 2, 5],
        email: ["a@test.com", "b@test.com", "c@test.com", "d@test.com", "a@test.com"],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: "number" },
        email: { type: "string" },
      },
      primaryKey: ["id"],
      uniqueKeys: [["email"]],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toEqual([
      { type: "row/primaryKey", rowNumber: 4, columnNames: ["id"] },
      { type: "row/uniqueKey", rowNumber: 5, columnNames: ["email"] },
    ])
  })

  it("should handle null values in unique keys correctly", async () => {
    const table = pl
      .DataFrame({
        id: [1, 2, null, 4, null, 2],
        name: ["Alice", "Bob", "Charlie", "David", "Eve", "Bob"],
      })
      .lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: ["number", "null"] as const },
        name: { type: "string" },
      },
      uniqueKeys: [["id"], ["id", "name"]],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toEqual([
      { type: "row/uniqueKey", rowNumber: 6, columnNames: ["id"] },
      { type: "row/uniqueKey", rowNumber: 6, columnNames: ["id", "name"] },
    ])
  })

  it("should skip unique keys with any null part", async () => {
    const table = pl
      .DataFrame({ a: [1, null, null, 1, null], b: ["x", "y", "y", null, null] })
      .lazy()
    const tableSchema: TableSchema = {
      properties: {
        a: { type: ["integer", "null"] },
        b: { type: ["string", "null"] },
      },
      uniqueKeys: [["a", "b"]],
    }

    expect(await inspectTable(table, { tableSchema })).toEqual([])
  })

  it("should skip unique keys with a missing value part", async () => {
    const table = pl.DataFrame({ code: ["NA", "a", "NA"] }).lazy()
    const tableSchema: TableSchema = {
      missingValues: ["NA"],
      properties: { code: { type: ["string", "null"] } },
      uniqueKeys: [["code"]],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toEqual([])
  })

  it("should report null primary key parts as missing cells", async () => {
    const table = pl.DataFrame({ a: [null, null, 1], b: [null, null, "z"] }).lazy()
    const tableSchema: TableSchema = {
      properties: {
        a: { type: ["integer", "null"] },
        b: { type: ["string", "null"] },
      },
      primaryKey: ["a", "b"],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toEqual([
      { type: "cell/missing", columnName: "a", rowNumber: 1, cell: "" },
      { type: "cell/missing", columnName: "a", rowNumber: 2, cell: "" },
      { type: "cell/missing", columnName: "b", rowNumber: 1, cell: "" },
      { type: "cell/missing", columnName: "b", rowNumber: 2, cell: "" },
    ])
  })

  it("should check a key column named like the probe count", async () => {
    const table = pl.DataFrame({ len: [1, 2, 1] }).lazy()
    const tableSchema: TableSchema = {
      properties: { len: { type: "integer" } },
      primaryKey: ["len"],
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(errors).toEqual([
      { type: "row/primaryKey", rowNumber: 3, columnNames: ["len"] },
    ])
  })

  it("should cap key errors per check", async () => {
    const table = pl
      .DataFrame({ id: Array(10).fill(1), name: Array(10).fill("a") })
      .lazy()
    const tableSchema: TableSchema = {
      properties: { id: { type: "integer" }, name: { type: "string" } },
      primaryKey: ["id"],
    }

    const errors = await inspectTable(table, { tableSchema, maxErrors: 4 })
    expect(errors.map(error => "rowNumber" in error && error.rowNumber)).toEqual([2, 3])
  })
})

describe("inspectTable (typed keys)", () => {
  it("should compare numbers as values", async () => {
    const table = pl.DataFrame({ id: ["1", "1.0", "2.5"] }).lazy()
    const tableSchema: TableSchema = {
      properties: { id: { type: "number" } },
      primaryKey: ["id"],
    }

    const errors = await inspectTable(table, { tableSchema })

    expect(errors).toEqual([
      { type: "row/primaryKey", rowNumber: 2, columnNames: ["id"] },
    ])
  })

  it("should find repeated keys that parse to one value", async () => {
    const table = pl.DataFrame({ flag: ["yes", "y"] }).lazy()
    const tableSchema: TableSchema = {
      properties: { flag: { type: "boolean", trueValues: ["yes", "y"] } },
      primaryKey: ["flag"],
    }

    const errors = await inspectTable(table, { tableSchema })

    expect(errors).toEqual([
      { type: "row/primaryKey", rowNumber: 2, columnNames: ["flag"] },
    ])
  })
})
