import { getHeaderRows, joinHeaderLabels } from "../../helpers/fileDialect.ts"
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

  for (const row of content) {
    const isCommentedRow = getIsCommentedRow(row, fileDialect)
    if (isCommentedRow) {
      continue
    }

    records.push(Object.fromEntries(labels.map((label, index) => [label, row[index]])))
  }

  return records
}

function getHeaderFromRows(
  rows: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  if (fileDialect?.columnNames) {
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
    if (row) {
      header.push(row)
    }
  }

  return header
}

function getContentFromRows(
  rows: DataRow[],
  fileDialect?: FileDialectWithHeaderAndCommentRows,
) {
  const headerRows = getHeaderRows(fileDialect)
  const commentRows = fileDialect?.commentRows ?? []
  const skipRows = headerRows[0] ? headerRows[0] - 1 : 0

  const content: DataRow[] = []
  for (const [index, row] of rows.entries()) {
    const number = index + 1

    if (number <= skipRows) {
      continue
    }

    if (headerRows.includes(number)) {
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
    return header[0].map(String)
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
