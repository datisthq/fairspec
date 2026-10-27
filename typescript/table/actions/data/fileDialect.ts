import {
  assertUniqueColumnNames,
  createLongRowError,
  getHeaderRows,
  joinHeaderLabels,
} from "../../helpers/fileDialect.ts"
import type { DataRecord, DataRow } from "../../models/data.ts"
import type { FileDialectWithHeaderAndCommentRows } from "../../models/fileDialect.ts"

export function getRecordsFromRows(
  rows: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  const records: DataRecord[] = []

  const header = getHeaderFromRows(rows, fileDialect)
  const content = getContentFromRows(rows, fileDialect)

  const labels = getLabelsFromHeader(header, fileDialect)
  if (!labels) {
    return records
  }
  if (!fileDialect?.columnNames) {
    assertUniqueColumnNames(labels)
  }

  for (const row of content) {
    const isCommentedRow = getIsCommentedRow(row, fileDialect)
    if (isCommentedRow) {
      continue
    }

    if (row.length > labels.length) {
      throw createLongRowError(row.length, labels.length)
    }

    records.push(Object.fromEntries(labels.map((label, index) => [label, row[index]])))
  }

  return records
}

/**
 * The column names the rows' own header rows give, whatever `columnNames` says, or
 * `undefined` without header rows, or none when the rows end before them; a multipart
 * resource compares them across parts
 */
export function getHeaderLabelsFromRows(
  rows: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  const headerRows = getHeaderRows(fileDialect)
  if (!headerRows.length) return undefined
  if (rows.length < Math.max(...headerRows)) return []
  const header = getHeaderFromRows(rows, fileDialect, { isOwnHeader: true })
  return getLabelsFromHeader(header, fileDialect) ?? []
}

function getHeaderFromRows(
  rows: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
  options?: { isOwnHeader?: boolean },
) {
  if (fileDialect?.columnNames && !options?.isOwnHeader) {
    return [fileDialect.columnNames]
  }

  const headerRows = getHeaderRows(fileDialect)

  if (!headerRows.length) {
    const length = Math.max(...rows.map(row => row.length))
    const labels = Array.from({ length }, (_, idx) => `column${idx + 1}`)

    return [labels]
  }

  const header: DataRow[] = []
  for (const number of headerRows) {
    const row = rows[number - 1]
    if (!row) {
      throw new Error(`Header row ${number} is past the end of the data`)
    }
    header.push(row)
  }

  return header
}

function getContentFromRows(
  rows: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  const headerRows = getHeaderRows(fileDialect)
  const commentRows = fileDialect?.commentRows ?? []
  const skipRows = Math.max(0, ...headerRows)

  const content: DataRow[] = []
  for (const [index, row] of rows.entries()) {
    const number = index + 1

    if (number <= skipRows) {
      continue
    }

    if (commentRows.includes(number)) {
      continue
    }

    const isCommentedRow = getIsCommentedRow(row, fileDialect)
    if (isCommentedRow) {
      continue
    }

    content.push(row)
  }

  return content
}

function getLabelsFromHeader(
  header: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  if (!header[0]) {
    return undefined
  }

  if (header.length === 1) {
    return header[0].map(cell =>
      cell === null || cell === undefined ? "" : String(cell),
    )
  }

  return joinHeaderLabels(header, fileDialect?.headerJoin ?? " ")
}

function getIsCommentedRow(
  row: unknown[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  const commentPrefix = fileDialect?.commentPrefix
  if (!commentPrefix) {
    return false
  }

  if (typeof row[0] !== "string") {
    return false
  }

  return row[0].startsWith(commentPrefix)
}
