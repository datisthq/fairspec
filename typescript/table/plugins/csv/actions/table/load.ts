import type { Resource } from "@fairspec/metadata"
import { getHeaderRows } from "../../../../helpers/fileDialect.ts"
import type { CsvFileDialect, TsvFileDialect } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { prefetchFiles } from "@fairspec/dataset"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { joinHeaderRows } from "../../../../actions/table/fileDialect.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import { skipCommentRows } from "../../../../actions/table/fileDialect.ts"
import type { Table } from "../../../../models/table.ts"
import * as pl from "nodejs-polars"
import { inferCsvFileDialect } from "../../actions/fileDialect/infer.ts"
import { getSupportedFileDialect } from "@fairspec/metadata"

// TODO: Condier using sample to extract header first
// for better commentPrefix + headerRows/commentRows support
// (consult with the Data Package Working Group)

export async function loadCsvTable(resource: Resource, options?: LoadTableOptions) {
  const maxBytes = options?.previewBytes
  const paths = await prefetchFiles(resource, { maxBytes })
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

  const scanOptions = getScanOptions(fileDialect)
  const isMultiHeader = getHeaderRows(fileDialect).length > 1 && !fileDialect?.columnNames
  if (isMultiHeader) {
    scanOptions.schema = getHeaderlessSchema(paths[0], scanOptions)
  }

  const tables: Table[] = []
  for (const path of paths) {
    const table = pl.scanCSV(path, scanOptions)
    tables.push(table)
  }

  // There is no way to specify column names in nodejs-polars by default
  // so we have to rename `column_*` to `column*` is table doesn't have header
  let table = pl.concat(tables)
  if (!scanOptions.hasHeader && !fileDialect?.columnNames && !isMultiHeader) {
    table = table.rename(
      Object.fromEntries(
        table.columns.map(name => [name, name.replace("column_", "column")]),
      ),
    )
  }

  if (fileDialect) {
    table = await joinHeaderRows(table, fileDialect)
    table = skipCommentRows(table, fileDialect)
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
    truncateRaggedLines: true,
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
  }

  return options
}

// Header rows are joined from the data, so every column is read as text and the
// widest of the first rows sets the width: a first header row may be shorter
function getHeaderlessSchema(
  path: string | undefined,
  options: Partial<pl.ScanCsvOptions>,
) {
  const columns = path
    ? pl.scanCSV(path, { ...options, inferSchemaLength: 100 }).columns
    : []
  return Object.fromEntries(columns.map(name => [name, pl.String]))
}
