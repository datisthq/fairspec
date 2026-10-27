import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"

describe("parseIntegerColumn", () => {
  it.each([
    ["1", 1],
    ["2", 2],
    ["1000", 1000],
    ["0", 0],
    ["00", null],
    ["01", null],
    ["000835", null],
    ["-0", 0],
    ["1.0", 1],
    ["1e3", 1000],
    ["1.50e1", 15],
    ["100e-2", 1],
    ["1e-1", null],
    ["+1", null],
    [" 1", null],
    ["1.", null],
    ["9223372036854775808", null],
    ["-9223372036854775809", null],
    ["1e19", null],
    ["1e400", null],
    ["0e99999999999999999999", 0],
    ["1e9223372036854775807", null],
    ["11e9223372036854775806", null],
    ["1.5e-9223372036854775808", null],
    ["", null],
    ["2.1", null],
    ["bad", null],
    ["0.0003", null],
    ["3.14", null],
    ["1/2", null],
  ])("default: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "integer",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["1", 1],
    ["1,000", 1000],
    ["1,000,000", 1000000],
    ["000,001", null],
  ])("groupChar ',': %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "integer",
          groupChar: ",",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["1 000", 1000],
    ["1'000'000", 1000000],
    ["1.000.000", 1000000],
  ])("groupChar other: %s -> %s", async (cell, expected) => {
    const groupChar = cell.includes(" ") ? " " : cell.includes("'") ? "'" : "."
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "integer",
          groupChar,
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["1", 1],
    ["1000", 1000],
    ["$1000", 1000],
    ["1000$", 1000],
    ["€1000", 1000],
    ["1000€", 1000],
    ["-12€", -12],
    ["€-12", -12],
    ["1,000", null],
  ])("bareNumber false: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "integer",
          withText: true,
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["$1,000,000", 1000000],
    ["1,000,000$", 1000000],
  ])("bareNumber false + groupChar: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "integer",
          groupChar: ",",
          withText: true,
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})

describe("stringifyIntegerColumn", () => {
  it.each([
    [1, "1"],
    [2, "2"],
    [1000, "1000"],
    [42, "42"],
    [-1, "-1"],
    [-100, "-100"],
    [0, "0"],
    [1234567890, "1234567890"],
    [-1234567890, "-1234567890"],
    // [null, ""],
  ])("default: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.Int64)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "integer",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})
