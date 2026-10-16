import { saveFile } from "@fairspec/dataset"
import type { JsonFileDialect, JsonlFileDialect } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { denormalizeTable } from "../../../../actions/table/denormalize.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import type { SaveTableOptions, Table } from "../../../../models/table.ts"
import { decodeJsonBuffer } from "../../actions/buffer/decode.ts"
import { encodeJsonBuffer } from "../../actions/buffer/encode.ts"
import { NATIVE_TYPES } from "../../settings.ts"

export async function saveJsonTable(table: Table, options: SaveTableOptions) {
  const { path, overwrite } = options

  const resource = { data: path, fileDialect: options.fileDialect }
  const fileDialect = await getSupportedFileDialect(resource, ["json", "jsonl"])
  if (!fileDialect) {
    throw new Error("Saving options is not compatible")
  }

  const isLines = fileDialect?.format === "jsonl"

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
  let buffer = frame.writeJSON({ format: isLines ? "lines" : "json" })
  let data = decodeJsonBuffer(buffer, { isLines })

  if (fileDialect) {
    data = processData(data, fileDialect)
  }

  buffer = encodeJsonBuffer(data, { isLines })
  await saveFile(path, buffer, { overwrite })

  return path
}

function processData(
  records: Record<string, any>[],
  dialect: JsonFileDialect | JsonlFileDialect,
) {
  let data: any = records

  if (dialect.columnNames) {
    const names = dialect.columnNames
    data = data.map((row: any) =>
      Object.fromEntries(names.map((name: any) => [name, row[name]])),
    )
  }

  if (dialect.rowType === "array") {
    const names = dialect.columnNames ?? Object.keys(data[0])
    data = [names, ...data.map((row: any) => names.map((name: any) => row[name]))]
  }

  if (dialect.format === "json") {
    if (dialect.jsonPointer) {
      // TODO: cover more jsonPointer cases
      data = { [dialect.jsonPointer]: data }
    }
  }

  return data
}
