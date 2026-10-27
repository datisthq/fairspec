import type { HeaderError } from "@fairspec/metadata"
import { createReport, FairspecException } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import {
  assertUniqueColumnNames,
  getHeaderRows,
  joinHeaderLabels,
} from "../../helpers/fileDialect.ts"
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

  // Every row up to the last header row is consumed by the header, listed or not
  const skippedRows = (headerRows[0] ?? 1) - 1
  const lastHeaderRow = headerRows.at(-1) ?? 1
  const headerHeight = lastHeaderRow - skippedRows - (dialect?.columnNames ? 1 : 0)
  const headerFrame = await table.head(headerHeight).collect()
  if (headerFrame.height < headerHeight) {
    throw new Error(`Header row ${lastHeaderRow} is past the end of the file`)
  }
  if (dialect?.columnNames) {
    return dropFirstRows(table, headerHeight)
  }

  const header = headerRows.map(number => headerFrame.row(number - skippedRows - 1) ?? [])
  const labels = joinHeaderLabels(header, dialect?.headerJoin ?? " ")
  const names = table.columns.map((name, index) => labels[index] ?? name)
  assertUniqueColumnNames(names)

  const mapping = Object.fromEntries(
    table.columns.map((name, index) => [name, names[index] ?? name]),
  )

  return dropFirstRows(table, lastHeaderRow - skippedRows).rename(mapping)
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

function dropFirstRows(table: Table, count: number) {
  return table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(NUMBER_COLUMN_NAME).gt(count))
    .drop(NUMBER_COLUMN_NAME)
}

function dropRows(table: Table, numbers: number[], offset: number) {
  return table
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(NUMBER_COLUMN_NAME).add(offset).isIn(numbers).not())
    .drop(NUMBER_COLUMN_NAME)
}

/**
 * Concatenates the tables of a multipart resource's parts; under a dialect with header
 * rows every part's header gives the first part's column names, else a `file/header`
 */
export function concatPartTables(
  tables: Table[],
  dialect?: FileDialectWithHeaderAndCommentRows,
) {
  const firstNames = tables[0]?.columns ?? []
  if (getHeaderRows(dialect).length) {
    tables.forEach((table, index) => {
      const names = table.columns
      const isSame =
        names.length === firstNames.length &&
        names.every((name, position) => name === firstNames[position])
      if (!isSame) throw createPartHeaderException(index + 1, names)
    })
  }
  return pl.concat(tables)
}

/**
 * The error of a part whose header differs from the first part's, or that has none
 */
export function createPartHeaderException(partNumber: number, columnNames: string[]) {
  const error: HeaderError = { type: "file/header", partNumber, columnNames }
  return new FairspecException(`Part ${partNumber} has another header than part 1`, {
    report: createReport([error]),
  })
}

/**
 * Tells the error of a part whose header differs, which a validator reports
 */
export function getIsPartHeaderException(error: unknown) {
  return (
    error instanceof FairspecException &&
    !!error.report?.errors.length &&
    error.report.errors.every(item => item.type === "file/header")
  )
}
