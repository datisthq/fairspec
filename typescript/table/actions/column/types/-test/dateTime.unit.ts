import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"

describe("parseDateTimeColumn", () => {
  it.each([
    ["2014-01-01T06:00:00Z", new Date(Date.UTC(2014, 0, 1, 6, 0, 0))],
    ["2014-01-01T06:00:00+00:00", new Date(Date.UTC(2014, 0, 1, 6, 0, 0))],
    ["2014-01-01T06:00:00-05:00", new Date(Date.UTC(2014, 0, 1, 11, 0, 0))],
    ["2014-01-01T01:00:00+02:00", new Date(Date.UTC(2013, 11, 31, 23, 0, 0))],
    ["2014-01-01T06:00:00.25Z", new Date(Date.UTC(2014, 0, 1, 6, 0, 0, 250))],
    ["2014-01-01t06:00:00z", new Date(Date.UTC(2014, 0, 1, 6, 0, 0))],
    ["2016-12-31T23:59:60Z", new Date(Date.UTC(2016, 11, 31, 23, 59, 59))],
    ["2016-12-31T12:30:60Z", null],
    ["2014-01-01 06:00:00Z", null],
    ["2014-01-01T06:00:00", null],
    ["2014-01-01T06:00:00+0200", null],
    ["2014-01-01T06:00:00Zx", null],
    ["x2014-01-01T06:00:00Z", null],
    ["2014-01-01T06:00Z", null],
    ["2014-02-30T06:00:00Z", null],
    ["2014-01-01", null],
    ["invalid", null],
    ["", null],
  ])("default: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "date-time",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it.each([
    ["21/11/2006 16:30+0200", new Date(Date.UTC(2006, 10, 21, 14, 30))],
    ["21/11/2006 16:30+02:00", new Date(Date.UTC(2006, 10, 21, 14, 30))],
    ["21/11/2006 16:30", null],
    ["21/11/2006 16:30Z", null],
    ["31/11/2006 16:30+0200", null],
    ["x21/11/2006 16:30+0200", null],
    ["invalid", null],
    ["", null],
  ])("temporalFormat: %s -> %s", async (cell, expected) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "date-time",
          temporalFormat: "%d/%m/%Y %H:%M%z",
        },
      },
    }

    const result = await normalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})

describe("stringifyDateTimeColumn", () => {
  it.each([
    [new Date(Date.UTC(2014, 0, 1, 6, 0, 0)), "2014-01-01T06:00:00Z"],
    [new Date(Date.UTC(2006, 10, 21, 16, 30, 0, 500)), "2006-11-21T16:30:00.500Z"],
  ])("default: %s -> %s", async (value, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.Datetime)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "date-time",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })

  it("writes a zoned value in UTC", async () => {
    const table = pl
      .DataFrame([
        pl.Series("name", [new Date(Date.UTC(2014, 0, 1, 6, 0, 0))], pl.Datetime),
      ])
      .lazy()
      .select(pl.col("name").dt.replaceTimeZone("UTC").dt.convertTimeZone("Europe/Rome"))
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "date-time",
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    expect(frame.toRecords()[0]?.name).toEqual("2014-01-01T06:00:00Z")
  })

  it.each([
    [
      new Date(Date.UTC(2006, 10, 21, 16, 30, 0)),
      "%d/%m/%Y %H:%M%z",
      "21/11/2006 16:30+0000",
    ],
    [
      new Date(Date.UTC(2006, 10, 21, 16, 30, 0)),
      "%d/%m/%Y %H:%M%:z",
      "21/11/2006 16:30+00:00",
    ],
  ])("temporalFormat: %s in %s -> %s", async (value, temporalFormat, expected) => {
    const table = pl.DataFrame([pl.Series("name", [value], pl.Datetime)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "date-time",
          temporalFormat,
        },
      },
    }

    const result = await denormalizeTable(table, tableSchema)
    const frame = await result.collect()

    const actual = frame.toRecords()[0]?.name
    expect(actual).toEqual(expected)
  })
})
