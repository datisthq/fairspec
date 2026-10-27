import type { Resource } from "@fairspec/metadata"
import { getHeaderRows } from "../../../../helpers/fileDialect.ts"
import type { CsvFileDialect, TsvFileDialect } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { prefetchFiles } from "@fairspec/dataset"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { stat } from "node:fs/promises"
import {
  concatPartTables,
  createPartHeaderException,
  joinHeaderRows,
} from "../../../../actions/table/fileDialect.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import { skipCommentRows } from "../../../../actions/table/fileDialect.ts"
import type { Table } from "../../../../models/table.ts"
import * as pl from "nodejs-polars"
import { inferCsvFileDialect } from "../../actions/fileDialect/infer.ts"
import { NUMBER_COLUMN_NAME } from "../../../../settings.ts"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { restoreDelimiter, swapDelimiter } from "../../helpers/delimiter.ts"
import { cleanCsvRows } from "../../helpers/rows.ts"
import { assertUniqueColumnNames } from "../../../../helpers/fileDialect.ts"

export async function loadCsvTable(resource: Resource, options?: LoadTableOptions) {
  const maxBytes = options?.previewBytes
  let paths = await prefetchFiles(resource, { maxBytes })
  if (!paths.length) {
    throw new Error("Resource path is not defined")
  }

  let fileDialect = await getSupportedFileDialect(resource, ["csv", "tsv"])
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  // A declared dialect is binding: its absent properties take their defaults
  if (!resource.fileDialect) {
    fileDialect = await inferCsvFileDialect({ ...resource, data: paths[0] }, options)
  }
  const isHeaderless = !getHeaderRows(fileDialect).length
  const partSizes =
    paths.length > 1
      ? await Promise.all(paths.map(async path => (await stat(path)).size))
      : []
  const emptyPart = partSizes.indexOf(0)
  if (emptyPart !== -1 && !isHeaderless) {
    throw createPartHeaderException(emptyPart + 1, [])
  }
  // A headerless part with no bytes adds no records, and Polars refuses an empty file
  if (emptyPart !== -1 && partSizes.some(size => size > 0)) {
    paths = paths.filter((_, index) => partSizes[index] !== 0)
  }

  const swapped = fileDialect && (await swapDelimiter(paths, fileDialect, { maxBytes }))
  if (swapped) {
    paths = swapped.paths
    fileDialect = swapped.fileDialect
  }
  const cleaned = fileDialect && (await cleanCsvRows(paths, fileDialect, { maxBytes }))
  if (cleaned) {
    paths = cleaned.paths
    fileDialect = cleaned.fileDialect
  }

  const isMultiHeader = getHeaderRows(fileDialect).length > 1 && !fileDialect?.columnNames

  // Header and comment rows count the rows of each part, so each part reads its own
  const tables: Table[] = []
  for (const path of paths) {
    const partDialect = fileDialect && (await renumberPrefixedRows(path, fileDialect))
    const scanOptions = getScanOptions(partDialect)
    if (isMultiHeader) {
      scanOptions.schema = getHeaderlessSchema(path, scanOptions, partDialect)
      // nodejs-polars refuses a first row shorter than a given schema unless it may
      // truncate; cleanCsvRows has already failed on a long row
      scanOptions.truncateRaggedLines = true
    }

    if (scanOptions.hasHeader && !partDialect?.columnNames) {
      assertUniqueColumnNames(await readHeaderNames(path, scanOptions))
    }
    let table = pl.scanCSV(path, scanOptions)
    if (swapped) {
      table = restoreDelimiter(table, swapped.swap)
    }
    if (partDialect) {
      table = await joinHeaderRows(table, partDialect)
      table = skipCommentRows(table, partDialect)
    }
    tables.push(table)
  }

  let table = concatPartTables(tables, fileDialect)

  // There is no way to specify column names in nodejs-polars by default
  // so we have to rename `column_*` to `column*` is table doesn't have header
  if (isHeaderless && !fileDialect?.columnNames) {
    table = table.rename(
      Object.fromEntries(
        table.columns.map(name => [name, name.replace("column_", "column")]),
      ),
    )
  }

  if (!options?.denormalized) {
    let tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) tableSchema = await inferTableSchemaFromTable(table, options)
    table = await normalizeTable(table, tableSchema)
  }

  return table
}

function getScanOptions(fileDialect?: TsvFileDialect | CsvFileDialect) {
  const headerRows = getHeaderRows(fileDialect)

  const options: Partial<pl.ScanCsvOptions> = {
    inferSchemaLength: 0,
    truncateRaggedLines: false,
  }

  options.skipRows = headerRows[0] ? headerRows[0] - 1 : 0
  options.hasHeader =
    headerRows.length === 1 || (headerRows.length > 1 && !!fileDialect?.columnNames)
  // Polars takes a single eol character and drops the "\r" before a "\n" itself
  options.eolChar = (fileDialect?.lineTerminator ?? "\n").slice(-1)
  options.sep = fileDialect?.format === "csv" ? (fileDialect?.delimiter ?? ",") : "\t"
  options.quoteChar =
    fileDialect?.format === "csv" ? (fileDialect?.quoteChar ?? '"') : undefined
  options.nullValues = fileDialect?.nullSequence
  options.commentPrefix = fileDialect?.commentPrefix

  if (fileDialect?.columnNames) {
    options.schema = Object.fromEntries(
      fileDialect.columnNames.map(name => [name, pl.String]),
    )
    options.missingColumns = "insert"
  }

  return options
}

// Polars renames a repeated header name, so the names are read as a data row
async function readHeaderNames(path: string, options: Partial<pl.ScanCsvOptions>) {
  const frame = await pl
    .scanCSV(path, {
      ...options,
      hasHeader: false,
      schema: undefined,
      nullValues: undefined,
      truncateRaggedLines: true,
    })
    .head(1)
    .collect()
  return frame.columns.map(name => {
    const cell: unknown = frame.getColumn(name).get(0)
    return typeof cell === "string" ? cell : ""
  })
}

// Header rows are joined from the data, so every column is read as text and the
// widest header row sets the width: a first header row may be shorter, and a data
// row wider than every header row is a long row
function getHeaderlessSchema(
  path: string | undefined,
  options: Partial<pl.ScanCsvOptions>,
  fileDialect?: TsvFileDialect | CsvFileDialect,
) {
  const headerRows = getHeaderRows(fileDialect)
  const inferSchemaLength = Math.max(...headerRows) - Math.min(...headerRows) + 1
  const columns = path
    ? pl.scanCSV(path, { ...options, inferSchemaLength, truncateRaggedLines: true })
        .columns
    : []
  return Object.fromEntries(columns.map(name => [name, pl.String]))
}

// Polars drops prefixed rows before it skips or numbers any, while headerRows and
// commentRows count every row of the file: renumber them past the prefixed rows
async function renumberPrefixedRows(
  path: string | undefined,
  fileDialect: TsvFileDialect | CsvFileDialect,
) {
  const { headerRows, commentRows, commentPrefix } = fileDialect
  const lastRow = Math.max(0, ...(headerRows || []), ...(commentRows ?? []))
  if (!path || !commentPrefix || lastRow < 2) return fileDialect

  const table = pl.scanCSV(path, {
    ...getScanOptions(fileDialect),
    skipRows: 0,
    hasHeader: false,
    nullValues: undefined,
    commentPrefix: undefined,
    schema: undefined,
    truncateRaggedLines: true,
  })
  const frame = await table
    .head(lastRow)
    .withRowIndex(NUMBER_COLUMN_NAME, 1)
    .filter(pl.col(table.columns[0] ?? "").str.startsWith(commentPrefix))
    .select(NUMBER_COLUMN_NAME)
    .collect()
  const prefixedRows = frame.getColumn(NUMBER_COLUMN_NAME).toArray().map(Number)
  const renumberRow = (row: number) =>
    row - prefixedRows.filter(prefixed => prefixed < row).length

  return {
    ...fileDialect,
    headerRows: headerRows && [...new Set(headerRows.map(renumberRow))],
    commentRows: commentRows?.filter(row => !prefixedRows.includes(row)).map(renumberRow),
  }
}
