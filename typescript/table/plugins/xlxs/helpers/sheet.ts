import type { OdsFileDialect, XlsxFileDialect } from "@fairspec/metadata"
import type { Cell } from "calamine-node"
import { createReader } from "calamine-node"
import { assertSheetSize } from "./guard.ts"

export type SheetCell = string | number | boolean | Date | null

const reader = createReader({
  experimentalFormats: ["ods"],
  maxInputBytes: 0xffff_ffff,
})

/**
 * Reads the dialect's sheet as rows of cells, without trailing empty cells.
 * Returns undefined when the workbook has no such sheet.
 */
export async function readSheetRows(
  path: string,
  fileDialect: XlsxFileDialect | OdsFileDialect,
  options?: { maxRows?: number },
) {
  const maxRows = options?.maxRows
  const index = fileDialect.sheetNumber ? fileDialect.sheetNumber - 1 : 0

  await assertSheetSize(path)
  await using book = await reader.openFile(path)
  const name = fileDialect.sheetName ?? book.sheets[index]?.name
  if (!name || !book.sheets.some(sheet => sheet.name === name)) return undefined

  const readOptions = { includeMergedCells: false }
  if (!maxRows) {
    const sheet = await book.readSheet(name, readOptions)
    return sheet.rows.map(normalizeRow)
  }

  const rows: SheetCell[][] = []
  for await (const batch of book.readBatches(name, {
    ...readOptions,
    batchSize: maxRows,
  })) {
    rows.push(...batch.rows.map(normalizeRow))
    if (rows.length >= maxRows) break
  }

  return rows.slice(0, maxRows)
}

function normalizeRow(row: Cell[]) {
  const cells = row.map(normalizeCell)

  let length = cells.length
  while (length > 0 && cells[length - 1] === null) length--

  return cells.slice(0, length)
}

// Python's twin maps python-calamine values to the same cells: a time-only value
// lands on 1899-12-31 and a duration is a number of days
function normalizeCell(cell: Cell): SheetCell {
  if (cell === "") return null
  if (cell === null || typeof cell !== "object") return cell
  if (cell.kind === "datetime" && cell.calendar) {
    const [year = 0, month = 1, day = 1, ...time] = cell.calendar
    return new Date(Date.UTC(year, month - 1, day, ...time))
  }
  if (cell.kind === "duration" && typeof cell.value === "number") return cell.value
  return null
}
