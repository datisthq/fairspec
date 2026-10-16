import { saveFile } from "@fairspec/dataset"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { utils, write } from "xlsx"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import type { SaveTableOptions, Table } from "../../../../models/table.ts"
import { NATIVE_TYPES } from "../../settings.ts"

// Currently, we use slow non-rust implementation as in the future
// polars-rust might be able to provide a faster native implementation

export async function saveXlsxTable(table: Table, options: SaveTableOptions) {
  const { path, overwrite } = options

  const resource = { data: path, fileDialect: options.fileDialect }
  const fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
  if (!fileDialect) {
    throw new Error("Saving options is not compatible")
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

  const frame = await table.collect()
  const sheetName = fileDialect?.sheetName ?? "Sheet1"

  const sheet = utils.json_to_sheet(frame.toRecords())
  const book = utils.book_new()
  utils.book_append_sheet(book, sheet, sheetName)

  const bookType = fileDialect?.format === "ods" ? "ods" : "xlsx"
  const buffer = write(book, { type: "buffer", bookType })
  await saveFile(path, buffer, { overwrite })

  return path
}
