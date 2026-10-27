import type { OdsFileDialect, XlsxFileDialect } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import {
  assertUniqueColumnNames,
  getHeaderRows,
  joinHeaderLabels,
} from "../../../helpers/fileDialect.ts"
import type { SheetCell } from "./sheet.ts"

type SheetType = "number" | "boolean" | "date" | "string"

/**
 * Builds a table from sheet rows. Data starts after the last header row; a column
 * keeps its cells' type when they share one and becomes text when they mix.
 */
export function createTableFromSheetRows(
  rows: SheetCell[][],
  fileDialect: XlsxFileDialect | OdsFileDialect,
) {
  const headerRows = getHeaderRows(fileDialect)
  const dataStart = Math.max(0, ...headerRows)
  const content = rows.slice(dataStart)

  const labels =
    fileDialect.columnNames ?? getSheetLabels(rows, content, headerRows, fileDialect)
  if (!fileDialect.columnNames) assertUniqueColumnNames(labels)

  const width = Math.max(
    0,
    ...rows.slice(headerRows[0] ? headerRows[0] - 1 : 0).map(getRowWidth),
  )
  if (fileDialect.columnNames && width > fileDialect.columnNames.length) {
    throw new Error(`The sheet has ${width} columns but columnNames has ${labels.length}`)
  }

  const columns = labels.map((_, index) => content.map(row => row[index] ?? null))
  const types = columns.map(getColumnType)

  const commentRows = fileDialect.commentRows ?? []
  const commentPrefix = fileDialect.commentPrefix
  const kept = content.flatMap((row, index) => {
    if (commentRows.includes(dataStart + index + 1)) return []
    const first = row[0]
    if (commentPrefix && typeof first === "string" && first.startsWith(commentPrefix)) {
      return []
    }
    return [index]
  })

  const series = labels.map((label, index) => {
    const type = types[index] ?? "string"
    const column = columns[index] ?? []
    const values = kept.map(row => column[row] ?? null)
    return createSheetSeries(label, values, type)
  })

  // Polars names an unnamed series of a new frame by its position; an empty name stays
  const frame = pl.DataFrame(series)
  const names = Object.fromEntries(
    frame.columns.map((name, index) => [name, labels[index] ?? name]),
  )
  return frame.rename(names).lazy()
}

/**
 * The column names a sheet's own header rows give, whatever `columnNames` says
 */
export function getSheetHeader(
  rows: SheetCell[][],
  fileDialect: XlsxFileDialect | OdsFileDialect,
) {
  const headerRows = getHeaderRows(fileDialect)
  const content = rows.slice(Math.max(0, ...headerRows))
  return getSheetLabels(rows, content, headerRows, fileDialect)
}

/**
 * Formats a cell as the text a mixed-type column holds.
 */
export function formatSheetCell(cell: SheetCell) {
  if (cell === null) return null
  if (cell instanceof Date) return formatSheetDate(cell)
  return String(cell)
}

function getSheetLabels(
  rows: SheetCell[][],
  content: SheetCell[][],
  headerRows: number[],
  fileDialect: XlsxFileDialect | OdsFileDialect,
) {
  if (!headerRows.length) {
    const width = Math.max(0, ...content.map(row => row.length))
    return Array.from({ length: width }, (_, index) => `column${index + 1}`)
  }

  const header = headerRows.map(number => {
    const row = rows[number - 1]
    if (!row) throw new Error(`Header row ${number} is past the end of the sheet`)
    return row.map(cell => formatSheetCell(cell) ?? "")
  })

  if (header.length <= 1) return header[0] ?? []
  return joinHeaderLabels(header, fileDialect.headerJoin ?? " ")
}

function getRowWidth(row: SheetCell[]) {
  return row.findLastIndex(cell => cell !== null) + 1
}

function getColumnType(cells: SheetCell[]) {
  let type: SheetType | undefined
  for (const cell of cells) {
    if (cell === null) continue
    const cellType = getCellType(cell)
    if (type && type !== cellType) return "string"
    type = cellType
  }

  return type ?? "string"
}

function getCellType(cell: string | number | boolean | Date) {
  if (cell instanceof Date) return "date"
  if (typeof cell === "number") return "number"
  if (typeof cell === "boolean") return "boolean"
  return "string"
}

function createSheetSeries(name: string, values: SheetCell[], type: SheetType) {
  switch (type) {
    case "number":
      return pl.Series(name, values, pl.Float64)
    case "boolean":
      return pl.Series(name, values, pl.Bool)
    case "date":
      return pl.Series(name, values, pl.Datetime("ms"))
    default:
      return pl.Series(name, values.map(formatSheetCell), pl.String)
  }
}

function formatSheetDate(date: Date) {
  const text = date.toISOString().slice(0, -1)
  return text.endsWith(".000") ? text.slice(0, -4) : text
}
