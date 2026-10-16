import { assertLocalPathVacant } from "@fairspec/dataset"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { getHeaderRows } from "../../../../helpers/fileDialect.ts"
import type { SaveTableOptions, Table } from "../../../../models/table.ts"
import { NATIVE_TYPES } from "../../settings.ts"

export async function saveCsvTable(table: Table, options: SaveTableOptions) {
  const { path, overwrite } = options

  const resource = { data: path, fileDialect: options.fileDialect }
  const fileDialect = await getSupportedFileDialect(resource, ["csv", "tsv"])
  if (!fileDialect) {
    throw new Error("Saving options is not compatible")
  }

  const headerRows = getHeaderRows(fileDialect)

  if (!overwrite) {
    await assertLocalPathVacant(path)
  }

  const tableSchema =
    options.tableSchema ??
    (await inferTableSchemaFromTable(table, {
      ...options,
      keepStrings: true,
    }))

  table = await denormalizeTable(table, tableSchema, {
    nativeTypes: NATIVE_TYPES,
  })

  await table
    .sinkCSV(path, {
      maintainOrder: true,
      includeHeader: headerRows.length > 0,
      lineTerminator: fileDialect.lineTerminator ?? "\n",
      quoteChar: fileDialect.format === "csv" ? fileDialect.quoteChar : undefined,
      separator: fileDialect.format === "csv" ? (fileDialect?.delimiter ?? ",") : "\t",
    })
    .collect()

  return path
}
