import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { joinHeaderRows, skipCommentRows } from "../fileDialect.ts"

describe("joinHeaderRows", () => {
  it("should join two header rows with default space separator", async () => {
    const table = pl
      .DataFrame({
        column_1: ["first", "name", "Alice", "Bob"],
        column_2: ["last", "name", "Smith", "Jones"],
        column_3: ["contact", "email", "alice@example.com", "bob@example.com"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
      headerRows: [1, 2],
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual(["first name", "last name", "contact email"])
    expect(collected.rows()).toEqual([
      ["Alice", "Smith", "alice@example.com"],
      ["Bob", "Jones", "bob@example.com"],
    ])
  })

  it("should join two header rows with custom separator", async () => {
    const table = pl
      .DataFrame({
        column_1: ["user", "first", "Alice"],
        column_2: ["user", "last", "Smith"],
        column_3: ["meta", "created", "2023-01-01"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
      headerRows: [1, 2],
      headerJoin: "_",
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual(["user_first", "user_last", "meta_created"])
    expect(collected.rows()).toEqual([["Alice", "Smith", "2023-01-01"]])
  })

  it("should return table unchanged when only one header row", async () => {
    const table = pl
      .DataFrame({
        name: ["Alice", "Bob"],
        age: [30, 25],
        city: ["NYC", "LA"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
      headerRows: [1],
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual(["name", "age", "city"])
    expect(collected.height).toBe(2)
  })

  it("should return table unchanged when no header rows", async () => {
    const table = pl
      .DataFrame({
        field1: ["Alice", "Bob"],
        field2: [30, 25],
        field3: ["NYC", "LA"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual(["field1", "field2", "field3"])
    expect(collected.height).toBe(2)
  })

  it("should join three header rows after skipped rows", async () => {
    const table = pl
      .DataFrame({
        column_1: ["person", "user", "first", "Alice"],
        column_2: ["person", "user", "last", "Smith"],
        column_3: ["location", "address", "city", "NYC"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
      headerRows: [2, 3, 4],
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual([
      "person user first",
      "person user last",
      "location address city",
    ])
    expect(collected.rows()).toEqual([["Alice", "Smith", "NYC"]])
  })

  it("should carry a label forward across empty header cells", async () => {
    const table = pl
      .DataFrame({
        column_1: ["person", "first", "Alice"],
        column_2: [null, "last", "Smith"],
        column_3: ["location", "city", "NYC"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
      headerRows: [1, 2],
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual(["person first", "person last", "location city"])
    expect(collected.rows()).toEqual([["Alice", "Smith", "NYC"]])
  })

  it("should drop the other header rows with column names", async () => {
    const table = pl
      .DataFrame({
        key: ["id", "1"],
        label: ["name", "apple"],
      })
      .lazy()

    const result = await joinHeaderRows(table, {
      format: "csv",
      headerRows: [1, 2],
      columnNames: ["key", "label"],
    })

    const collected = await result.collect()
    expect(collected.columns).toEqual(["key", "label"])
    expect(collected.rows()).toEqual([["1", "apple"]])
  })
})

describe("skipCommentRows", () => {
  it("should skip comment rows by row number", async () => {
    const table = pl
      .DataFrame({
        name: ["Alice", "# Comment", "Bob", "Charlie"],
        age: [30, 0, 25, 35],
        city: ["NYC", "ignored", "LA", "SF"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
      commentRows: [2],
      headerRows: false,
    })

    const collected = await result.collect()
    expect(collected.height).toBe(3)
    expect(collected.row(0)).toEqual(["Alice", 30, "NYC"])
    expect(collected.row(1)).toEqual(["Bob", 25, "LA"])
    expect(collected.row(2)).toEqual(["Charlie", 35, "SF"])
  })

  it("should skip multiple comment rows", async () => {
    const table = pl
      .DataFrame({
        name: ["Alice", "# Comment 1", "Bob", "# Comment 2", "Charlie"],
        age: [30, 0, 25, 0, 35],
        city: ["NYC", "ignored", "LA", "ignored", "SF"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
      commentRows: [2, 4],
      headerRows: false,
    })

    const collected = await result.collect()
    expect(collected.height).toBe(3)
    expect(collected.row(0)).toEqual(["Alice", 30, "NYC"])
    expect(collected.row(1)).toEqual(["Bob", 25, "LA"])
    expect(collected.row(2)).toEqual(["Charlie", 35, "SF"])
  })

  it("should return table unchanged when no commentRows specified", async () => {
    const table = pl
      .DataFrame({
        name: ["Alice", "Bob", "Charlie"],
        age: [30, 25, 35],
        city: ["NYC", "LA", "SF"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
    })

    const collected = await result.collect()
    expect(collected.height).toBe(3)
    expect(collected.columns).toEqual(["name", "age", "city"])
  })

  it("should skip rows after header when headerRows specified", async () => {
    const table = pl
      .DataFrame({
        col1: ["name", "Alice", "# Comment", "Bob"],
        col2: ["age", "30", "-1", "25"],
        col3: ["city", "NYC", "ignored", "LA"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
      headerRows: [2],
      commentRows: [5],
    })

    const collected = await result.collect()
    expect(collected.height).toBe(3)
    expect(collected.row(0)).toEqual(["name", "age", "city"])
    expect(collected.row(1)).toEqual(["Alice", "30", "NYC"])
    expect(collected.row(2)).toEqual(["Bob", "25", "LA"])
  })

  it("should handle commentRows at the beginning", async () => {
    const table = pl
      .DataFrame({
        name: ["# Skip this", "Alice", "Bob"],
        age: [0, 30, 25],
        city: ["ignored", "NYC", "LA"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
      commentRows: [1],
      headerRows: false,
    })

    const collected = await result.collect()
    expect(collected.height).toBe(2)
    expect(collected.row(0)).toEqual(["Alice", 30, "NYC"])
    expect(collected.row(1)).toEqual(["Bob", 25, "LA"])
  })

  it("should handle commentRows at the end", async () => {
    const table = pl
      .DataFrame({
        name: ["Alice", "Bob", "# Footer comment"],
        age: [30, 25, 0],
        city: ["NYC", "LA", "ignored"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
      commentRows: [3],
      headerRows: false,
    })

    const collected = await result.collect()
    expect(collected.height).toBe(2)
    expect(collected.row(0)).toEqual(["Alice", 30, "NYC"])
    expect(collected.row(1)).toEqual(["Bob", 25, "LA"])
  })

  it("should handle multiple header rows with commentRows", async () => {
    const table = pl
      .DataFrame({
        col1: ["person", "first", "Alice", "# Comment", "Bob"],
        col2: ["person", "last", "Smith", "ignored", "Jones"],
        col3: ["location", "city", "NYC", "ignored", "LA"],
      })
      .lazy()

    const result = skipCommentRows(table, {
      format: "csv",
      headerRows: [2, 3],
      commentRows: [7],
    })

    const collected = await result.collect()
    expect(collected.height).toBe(4)
    expect(collected.row(0)).toEqual(["person", "person", "location"])
    expect(collected.row(1)).toEqual(["first", "last", "city"])
    expect(collected.row(2)).toEqual(["Alice", "Smith", "NYC"])
    expect(collected.row(3)).toEqual(["Bob", "Jones", "LA"])
  })
})
