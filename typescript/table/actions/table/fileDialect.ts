import * as pl from "nodejs-polars"
import { getHeaderRows, joinHeaderLabels } from "../../helpers/fileDialect.ts"
import type { FileDialectWithHeaderAndCommentRows } from "../../models/fileDialect.ts"
import type { Table } from "../../models/table.ts"
import { NUMBER_COLUMN_NAME } from "../../settings.ts"

// The table is read without a header, so its header rows are still data rows;
// with columnNames the first header row was read as the header and is gone
export async function joinHeaderRows(
  table: Table,
  dialect: FileDialectWithHeaderAndCommentRows,
) {
  const headerRows = getHeaderRows(dialect)
  if (headerRows.length < 2) {
    return table
  }

  const skippedRows = (headerRows[0] ?? 1) - 1
  if (dialect?.columnNames) {
    return dropRows(table, headerRows, skippedRows + 1)
  }

  const lastHeaderRow = headerRows.at(-1) ?? 1
  const headerFrame = await table.head(lastHeaderRow - skippedRows).collect()

  const header = headerRows.map(number => headerFrame.row(number - skippedRows - 1) ?? [])
  const labels = joinHeaderLabels(header, dialect?.headerJoin ?? " ")

  const mapping = Object.fromEntries(
    table.columns.map((name, index) => [name, labels[index] || name]),
  )

  return dropRows(table, headerRows, skippedRows).rename(mapping)
}

export function skipCommentRows(
  table: Table,
  dialect: FileDialectWithHeaderAndCommentRows,
) {
  const headerRows = getHeaderRows(dialect)
  const commentOffset = headerRows.at(-1) ?? 0
  if (!dialect?.commentRows) {
    return table
  }

  return dropRows(table, dialect.commentRows, commentOffset)
}

function dropRows(table: Table, numbers: number[], offset: number) {
  return table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(NUMBER_COLUMN_NAME).add(offset).isIn(numbers).not())
    .drop(NUMBER_COLUMN_NAME)
}
