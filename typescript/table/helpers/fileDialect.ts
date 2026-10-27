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

/**
 * Throws when a header gives two columns one name; an empty name is a name too
 */
export function assertUniqueColumnNames(names: string[]) {
  const seen = new Set<string>()
  for (const name of names) {
    if (seen.has(name)) throw new Error(`Column name "${name}" is repeated in the header`)
    seen.add(name)
  }
}

/**
 * Tells a row with more cells than the table has columns, which Polars raises while
 * reading, so a validator reports the file as not loadable instead of throwing
 */
export function getIsLongRowError(error: unknown) {
  return (
    error instanceof Error && error.message.includes("found more fields than defined")
  )
}
