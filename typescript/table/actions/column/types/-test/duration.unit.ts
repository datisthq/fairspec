import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { inspectTable } from "../../../../actions/table/inspect.ts"

describe("parseDurationColumn", () => {
  it.each([
    ["P23DT23H", true],
    ["P1Y2M3DT4H5M6S", true],
    ["PT30M", true],
    ["P1D", true],
    ["PT1H", true],
    ["P1W", true],
    ["PT0S", true],
    ["ghijkl", false],
    ["0x1234", false],
    ["hello world", false],
    ["PT", false],
    ["P1DT", false],
    ["p1dt", false],
    ["p1dt2h", true],
    ["-P1D", false],
    ["P1W2D", false],
    ["PT1.5S", false],
    ["P1Y1D", false],
  ])("%s -> %s", async (cell, valid) => {
    const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: {
          type: "string",
          format: "duration",
        },
      },
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(!errors.length).toEqual(valid)
  })

  it("checks the string constraints of a readable cell", async () => {
    const table = pl
      .DataFrame([pl.Series("name", ["P1D", "P22D", "1D"], pl.String)])
      .lazy()
    const tableSchema: TableSchema = {
      properties: {
        name: { type: "string", format: "duration", enum: ["P1D"], maxLength: 3 },
      },
    }

    const errors = await inspectTable(table, { tableSchema })
    expect(
      errors.map(error => [error.type, "rowNumber" in error && error.rowNumber]),
    ).toEqual([
      ["cell/enum", 2],
      ["cell/maxLength", 2],
      ["cell/type", 3],
    ])
  })
})
