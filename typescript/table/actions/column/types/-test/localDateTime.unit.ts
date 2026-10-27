import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"

describe("parseLocalDateTimeColumn", () => {
  it.each([
    ["2014-01-01T06:00:00", new Date(Date.UTC(2014, 0, 1, 6, 0, 0))],
    ["2014-01-01T06:00:00.25", new Date(Date.UTC(2014, 0, 1, 6, 0, 0, 250))],
    ["2014-01-01t06:00:00", new Date(Date.UTC(2014, 0, 1, 6, 0, 0))],
    ["2016-12-31T23:59:60", new Date(Date.UTC(2016, 11, 31, 23, 59, 59))],
    ["2016-12-31T12:30:60", null],
    ["2014-01-01T06:00:00Z", null],
    ["2014-01-01T06:00:00-05:00", null],
    ["2014-01-01T06:00:00x", null],
    ["2014-01-01T06:00", null],
    ["2014-01-01", null],
    ["Mon 1st Jan 2014 9 am", null],
    ["invalid", null],
    ["", null],
    ["2014-1-01T06:00:00", null],
    ["2014-01-01T6:00:00", null],
    [" 2014-01-01T06:00:00", null],
    ["+2014-01-01T06:00:00", null],
  ])("default: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "local-date-time",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["21/11/2006 16:30", new Date(Date.UTC(2006, 10, 21, 16, 30))],
    ["21/11/2006 16:30:00", null],
    ["16:30 21/11/06", null],
    ["invalid", null],
    ["", null],
  ])("temporalFormat: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "local-date-time",
          temporalFormat: "%d/%m/%Y %H:%M",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([["21/11/06 16:30", null]])(
    "invalid temporalFormat: %s -> %s",
    async (cell, expected) => {
      const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
      const tableSchema: TableSchema = {
        properties: {
          name: {
            type: "string",
            format: "local-date-time",
            temporalFormat: "invalid",
          },
        },
      }

      const result = await normalizeTable(table, tableSchema)
      const frame = await result.collect()

      const actual = frame.toRecords()[0]?.name
      expect(actual).toEqual(expected)
    },
  )
})

describe("stringifyLocalDateTimeColumn", () => {
  it.each([
    [new Date(Date.UTC(2014, 0, 1, 6, 0, 0)), "2014-01-01T06:00:00"],
    [new Date(Date.UTC(2006, 10, 21, 16, 30, 0, 500)), "2006-11-21T16:30:00.500"],
  ])("default: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.Datetime)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "local-date-time",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([[new Date(Date.UTC(2006, 10, 21, 16, 30, 0)), "21/11/2006 16:30"]])(
    "temporalFormat %%d/%%m/%%Y %%H:%%M: %s -> %s",
    async (value, expected) => {
      const table = pl.DataFrame([pl.Series("name", [value], pl.Datetime)]).lazy()
      const tableSchema: TableSchema = {
        properties: {
          name: {
            type: "string",
            format: "local-date-time",
            temporalFormat: "%d/%m/%Y %H:%M",
          },
        },
      }

      const result = await denormalizeTable(table, tableSchema)
      const frame = await result.collect()

      const actual = frame.toRecords()[0]?.name
      expect(actual).toEqual(expected)
    },
  )

  it.each([[new Date(Date.UTC(2014, 0, 1, 6, 0, 0)), "2014/01/01T06:00:00"]])(
    "temporalFormat %%Y/%%m/%%dT%%H:%%M:%%S: %s -> %s",
    async (value, expected) => {
      const table = pl.DataFrame([pl.Series("name", [value], pl.Datetime)]).lazy()
      const tableSchema: TableSchema = {
        properties: {
          name: {
            type: "string",
            format: "local-date-time",
            temporalFormat: "%Y/%m/%dT%H:%M:%S",
          },
        },
      }

      const result = await denormalizeTable(table, tableSchema)
      const frame = await result.collect()

      const actual = frame.toRecords()[0]?.name
      expect(actual).toEqual(expected)
    },
  )
})
