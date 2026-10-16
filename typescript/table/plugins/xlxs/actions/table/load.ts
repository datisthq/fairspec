import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { loadFile, prefetchFiles } from "@fairspec/dataset"
import type { DataRow } from "../../../../models/data.ts"
import { getRecordsFromRows } from "../../../../actions/data/fileDialect.ts"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import type { Table } from "../../../../models/table.ts"
import * as pl from "nodejs-polars"
import { read, utils } from "xlsx"
import { inferXlsxFileDialect } from "../fileDialect/infer.ts"

// Currently, we use slow non-rust implementation as in the future
// polars-rust might be able to provide a faster native implementation

export async function loadXlsxTable(resource: Resource, options?: LoadTableOptions) {
  const paths = await prefetchFiles(resource)
  if (!paths.length) {
    throw new Error("Resource path is not defined")
  }

  let fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  // TODO: Consider inferring all the missing dialect properties
  if (!fileDialect || Object.keys(fileDialect).length <= 1) {
    fileDialect = await inferXlsxFileDialect({ ...resource, data: paths[0] })
  }

  const tables: Table[] = []
  for (const path of paths) {
    const buffer = await loadFile(path)

    const book = read(buffer, { type: "buffer" })
    const sheetIndex = fileDialect?.sheetNumber ? fileDialect.sheetNumber - 1 : 0
    const sheetName = fileDialect?.sheetName ?? book.SheetNames[sheetIndex]
    const sheet = sheetName ? book.Sheets[sheetName] : undefined

    if (sheet) {
      const rows = utils.sheet_to_json(sheet, {
        header: 1,
        raw: true,
      }) as DataRow[]

      const records = getRecordsFromRows(rows, fileDialect)
      const table = pl.DataFrame(records).lazy()

      tables.push(table)
    }
  }

  let table = pl.concat(tables)

  if (!options?.denormalized) {
    let tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) tableSchema = await inferTableSchemaFromTable(table, options)
    table = await normalizeTable(table, tableSchema)
  }

  return table
}
