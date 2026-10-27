import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { prefetchFiles } from "@fairspec/dataset"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import type { Table } from "../../../../models/table.ts"
import { concatPartTables } from "../../../../actions/table/fileDialect.ts"
import { readSheetRows } from "../../helpers/sheet.ts"
import { createTableFromSheetRows, getSheetHeader } from "../../helpers/table.ts"
import { inferXlsxFileDialect } from "../fileDialect/infer.ts"

export async function loadXlsxTable(resource: Resource, options?: LoadTableOptions) {
  const paths = await prefetchFiles(resource)
  if (!paths.length) {
    throw new Error("Resource path is not defined")
  }

  let fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  // A declared dialect is binding: its absent properties take their defaults
  if (!resource.fileDialect) {
    fileDialect =
      (await inferXlsxFileDialect({ ...resource, data: paths[0] })) ?? fileDialect
  }

  // With columnNames the tables carry those names, so each part's own header is compared
  const isOwnHeader = paths.length > 1 && !!fileDialect.columnNames
  const tables: Table[] = []
  const partHeaders: string[][] = []
  for (const path of paths) {
    const rows = await readSheetRows(path, fileDialect)

    if (rows) {
      tables.push(createTableFromSheetRows(rows, fileDialect))
      if (isOwnHeader) partHeaders.push(getSheetHeader(rows, fileDialect))
    }
  }

  let table = concatPartTables(tables, fileDialect, isOwnHeader ? partHeaders : undefined)

  if (!options?.denormalized) {
    let tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) tableSchema = await inferTableSchemaFromTable(table, options)
    table = await normalizeTable(table, tableSchema)
  }

  return table
}
