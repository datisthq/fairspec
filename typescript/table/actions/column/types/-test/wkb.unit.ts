import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { inspectTable } from "../../../../actions/table/inspect.ts"

const POINT = "0101000000000000000000f03f0000000000000040"
const zeros = (count: number) => "00".repeat(count)

async function getIsValid(cell: string) {
  const table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
  const tableSchema: TableSchema = {
    properties: {
      name: {
        type: "string",
        format: "wkb",
      },
    },
  }

  const errors = await inspectTable(table, { tableSchema })
  return !errors.length
}

describe("parseWkbColumn", () => {
  it.each([
    [POINT, true],
    ["ghijkl", false],
    ["0x1234", false],
    ["hello world", false],
  ])("%s -> %s", async (cell, valid) => {
    expect(await getIsValid(cell)).toEqual(valid)
  })

  // Expected verdicts are shapely 2.1 (GEOS 3.13) `from_wkb(bytes.fromhex(...))` raising or not
  it.each([
    [POINT.toUpperCase(), true],
    [`${POINT}00`, true],
    [POINT.slice(0, -2), false],
    ["01", false],
    [`01 ${POINT.slice(2)}`, true],
    [`\t${POINT}\n`, true],
    [POINT.slice(0, 3), false],
    ["0000000001" + "3ff0000000000000" + "4000000000000000", true],
    [`01e9030000${zeros(24)}`, true],
    [`0101000080${zeros(24)}`, true],
    [`0101000040${zeros(24)}`, true],
    [`01d1070000${zeros(24)}`, true],
    [`01b90b0000${zeros(32)}`, true],
    [`0101000020e6100000${zeros(16)}`, true],
    [`010200000001000000${zeros(16)}`, false],
    ["010200000000000000", true],
    [
      `01030000000100000003000000${zeros(16)}000000000000f03f${zeros(8)}${zeros(16)}`,
      true,
    ],
    [
      `01030000000100000003000000${zeros(16)}000000000000f03f${zeros(8)}000000000000f03f${zeros(8)}`,
      false,
    ],
    [`010400000001000000${POINT}`, true],
    ["010400000001000000010200000000000000", false],
    [`010700000001000000${POINT}`, true],
    ["010800000000000000", false],
    ["0101000000000000000000f87f000000000000f87f", true],
    ["010f00000000000000", false],
    ["0102000000ffffffff", false],
    ["010300000000000000", true],
    ["010600000001000000010300000000000000", true],
  ])("matches shapely: %j -> %s", async (cell, valid) => {
    expect(await getIsValid(cell)).toEqual(valid)
  })

  it.each([
    [`02${POINT.slice(2)}`, false],
    [`ff${POINT.slice(2)}`, false],
    [`010400000001000000${POINT}`, true],
    [`010400000001000000${`02${POINT.slice(2)}`}`, false],
  ])("refuses an unknown byte order: %s -> %s", async (cell, valid) => {
    expect(await getIsValid(cell)).toEqual(valid)
  })
})
