import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"

describe("parseListColumn", () => {
  it.each([
    ["a,b,c", ["a", "b", "c"]],
    ["1,2,3", ["1", "2", "3"]],
    ["foo,bar,baz", ["foo", "bar", "baz"]],
    ["single", ["single"]],
    [" a", [" a"]],
    ["", []],
    ["a,,c", ["a", "", "c"]],
    [",b,", ["", "b", ""]],
    [",,,", ["", "", "", ""]],
  ])("default: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["1,2,3", [1, 2, 3]],
    ["0,-1,42", [0, -1, 42]],
    ["-10,0,10", [-10, 0, 10]],
    ["42", [42]],
    ["1,,3", null],
    [",2,", null],
    ["1,a,3", null],
    ["1.5,2,3", null],
    ["1,2.0", [1, 2]],
    ["1,+2", null],
    ["1, 2", null],
    ["", []],
  ])("items integer: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          itemType: "integer",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["1.5,2.1,3.7", [1.5, 2.1, 3.7]],
    ["0,-1.1,42", [0, -1.1, 42]],
    ["-10.5,0,10", [-10.5, 0, 10]],
    ["3.14", [3.14]],
    ["1.1,,3.3", null],
    [",2.2,", null],
    ["1,.5", null],
    ["1,NaN", null],
    ["1.1,a,3.3", null],
  ])("items number: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          itemType: "number",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["a;b;c", ["a", "b", "c"]],
    ["1;2;3", ["1", "2", "3"]],
    ["single", ["single"]],
    ["a;;c", ["a", "", "c"]],
  ])("delimiter ';': %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          delimiter: ";",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["2024-01-15,2024-02-29", true],
    ["2024-01-15,2024-1-5", false],
    ["2024-01-15, 2024-01-16", false],
  ])("items date: %s -> %s", async (cell, valid) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          itemType: "date",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual !== null).toEqual(valid)
  })
})

describe("stringifyListColumn", () => {
  it.each([
    [["a", "b", "c"], "a,b,c"],
    [["foo", "bar", "baz"], "foo,bar,baz"],
    [["1", "2", "3"], "1,2,3"],
    [["single"], "single"],
    [["a", "", "c"], "a,,c"],
    [["", "b", ""], ",b,"],
    [["", "", "", ""], ",,,"],
    [[null, "b", null], "b"],
    [["a", null, "c"], "a,c"],
    [[], ""],
  ])("default: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.String))]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    [[1, 2, 3], "1,2,3"],
    [[0, -1, 42], "0,-1,42"],
    [[-10, 0, 10], "-10,0,10"],
    [[42], "42"],
    [[1, null, 3], "1,3"],
    [[null, 2, null], "2"],
    [[], ""],
  ])("items integer: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.Int16))]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          itemType: "integer",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    [[1.5, 2.1, 3.7], "1.5,2.1,3.7"],
    [[0, -1.1, 42], "0.0,-1.1,42.0"],
    [[-10.5, 0, 10], "-10.5,0.0,10.0"],
    [[3.14], "3.14"],
    [[1.1, null, 3.3], "1.1,3.3"],
    [[null, 2.2, null], "2.2"],
    [[], ""],
  ])("items number: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.Float64))]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          itemType: "number",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    [["a", "b", "c"], "a;b;c"],
    [["1", "2", "3"], "1;2;3"],
    [["single"], "single"],
    [["a", "", "c"], "a;;c"],
    [["", "b", ""], ";b;"],
    [[], ""],
  ])("delimiter ';': %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.String))]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "list",
          delimiter: ";",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})
