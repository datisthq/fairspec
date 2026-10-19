import type { DataRow } from "../models/data.ts"
import type { FileDialectWithHeaderAndCommentRows } from "../models/fileDialect.ts"

export function getHeaderRows(fileDialect?: FileDialectWithHeaderAndCommentRows) {
  return fileDialect?.headerRows !== false ? (fileDialect?.headerRows ?? [1]) : []
}

// A label carries forward across the empty cells to its right, so a group label
// spanning several columns prefixes each of them
export function joinHeaderLabels(header: DataRow[], headerJoin: string) {
  const width = Math.max(0, ...header.map(row => row.length))
  const rows = header.map(row => fillHeaderRow(row, width))

  return Array.from({ length: width }, (_, index) =>
    rows
      .map(row => row[index])
      .filter(Boolean)
      .join(headerJoin),
  )
}

function fillHeaderRow(row: DataRow, width: number) {
  const labels: string[] = []

  let label = ""
  for (let index = 0; index < width; index++) {
    const cell = row[index]
    if (cell !== undefined && cell !== null && String(cell) !== "") label = String(cell)
    labels.push(label)
  }

  return labels
}
