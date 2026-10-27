import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"

describe("parseTimeColumn", () => {
  it.each([
    ["06:00:00Z", "06:00:00"],
    ["06:00:00+02:00", "04:00:00"],
    ["01:00:00+02:00", "23:00:00"],
    ["06:00:00.5-01:00", "07:00:00.500"],
    ["06:00:00z", "06:00:00"],
    ["23:59:60Z", "23:59:59"],
    ["15:59:60-08:00", "23:59:59"],
    ["12:30:60Z", null],
    ["23:59:60+01:00", null],
    ["06:00:00", null],
    ["06:00:00+0200", null],
    ["25:00:00Z", null],
    ["09:00Z", null],
    ["invalid", null],
    ["", null],
  ])("default: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "time",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["06:00+02:00", "04:00:00"],
    ["3:00+0000", "03:00:00"],
    ["06:00", null],
    ["06:00:00+02:00", null],
    ["invalid", null],
    ["", null],
  ])("temporalFormat: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "time",
          temporalFormat: "%H:%M%:z",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})

describe("stringifyTimeColumn", () => {
  it.each([
    [new Date(Date.UTC(2014, 0, 1, 6, 0, 0)), "06:00:00Z"],
    [new Date(Date.UTC(2014, 0, 1, 16, 30, 0)), "16:30:00Z"],
  ])("default: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.Time)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "time",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    [new Date(Date.UTC(2014, 0, 1, 6, 0, 0)), "06:00+00:00"],
    [new Date(Date.UTC(2014, 0, 1, 16, 30, 0)), "16:30+00:00"],
  ])("temporalFormat: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.Time)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "time",
          temporalFormat: "%H:%M%:z",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})
