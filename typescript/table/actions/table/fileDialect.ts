import * as pl from "nodejs-polars"
import { getHeaderRows } from "../../helpers/fileDialect.ts"
import type { FileDialectWithHeaderAndCommentRows } from "../../models/fileDialect.ts"
import type { Table } from "../../models/table.ts"
import { NUMBER_COLUMN_NAME } from "../../settings.ts"

export async function joinHeaderRows(
  table: Table,
  dialect: FileDialectWithHeaderAndCommentRows,
) {
  const headerRows = getHeaderRows(dialect)
  const headerOffset = headerRows.at(0) ?? 0
  const headerJoin = dialect?.headerJoin ?? " "
  if (headerRows.length < 2) {
    return table
  }

  const extraLabelsFrame = await table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(NUMBER_COLUMN_NAME).add(headerOffset).isIn(headerRows))
    .select(...table.columns.map(name => pl.col(name).str.concat(headerJoin)))
    .collect()

  const labels = table.columns
  const extraLabels = extraLabelsFrame.row(0)

  const mapping = Object.fromEntries(
    labels.map((label, index) => [label, [label, extraLabels[index]].join(headerJoin)]),
  )

  return table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(NUMBER_COLUMN_NAME).add(headerOffset).isIn(headerRows).not())
    .rename(mapping)
    .drop(NUMBER_COLUMN_NAME)
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

  return table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(NUMBER_COLUMN_NAME).add(commentOffset).isIn(dialect.commentRows).not())
    .drop(NUMBER_COLUMN_NAME)
}
