import type { Resource } from "@fairspec/metadata"
import type { JsonFileDialect, JsonlFileDialect } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { loadFile, prefetchFiles } from "@fairspec/dataset"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import { getRecordsFromRows } from "../../../../actions/data/fileDialect.ts"
import { getIsObject } from "../../../../helpers/general.ts"
import type { Table } from "../../../../models/table.ts"
import * as pl from "nodejs-polars"
import { decodeJsonBuffer } from "../../actions/buffer/decode.ts"
import { parseJsonPointer } from "../../helpers/pointer.ts"
import { getSupportedFileDialect } from "@fairspec/metadata"
import { inferJsonFileDialect } from "../fileDialect/infer.ts"

/**
 * Load a JSON or JSONL file as a table, or `undefined` when the rows it points
 * at are not an array (a JSON file whose top level is an object is not a table)
 */
export async function loadJsonTable(resource: Resource, options?: LoadTableOptions) {
  let fileDialect = await getSupportedFileDialect(resource, ["json", "jsonl"])
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  const maxBytes = fileDialect?.format === "jsonl" ? options?.previewBytes : undefined
  const paths = await prefetchFiles(resource, { maxBytes })
  if (!paths.length) {
    throw new Error("Resource data is not defined")
  }

  // TODO: Consider inferring all the missing dialect properties
  if (!fileDialect || Object.keys(fileDialect).length <= 1) {
    fileDialect = await inferJsonFileDialect({ ...resource, data: paths[0] })
  }
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  const isLines = fileDialect.format === "jsonl"
  const isDefault =
    Object.keys(fileDialect).filter(
      key => !["type", "title", "description"].includes(key),
    ).length === 0

  const tables: Table[] = []
  for (const path of paths) {
    if (isLines && isDefault) {
      const table = pl.scanJson(path)
      tables.push(table)
      continue
    }

    const buffer = await loadFile(path)
    const records = getRecordsFromJson(decodeJsonBuffer(buffer, { isLines }), fileDialect)
    if (!records) return undefined

    const table = pl.DataFrame(records).lazy()
    tables.push(table)
  }

  let table = pl.concat(tables)

  if (!options?.denormalized) {
    let tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) tableSchema = await inferTableSchemaFromTable(table, options)
    table = await normalizeTable(table, tableSchema)
  }

  return table
}

function getRecordsFromJson(
  document: unknown,
  dialect: JsonFileDialect | JsonlFileDialect,
) {
  const data =
    dialect.format === "json" && dialect.jsonPointer !== undefined
      ? getJsonPointerValue(document, dialect.jsonPointer)
      : document
  if (!Array.isArray(data)) return undefined

  const rowType = dialect.rowType ?? (Array.isArray(data[0]) ? "array" : "object")
  if (rowType === "array") {
    return getRecordsFromRows(data.filter(Array.isArray), dialect)
  }

  const records = data.filter(getIsObject)
  const columnNames = dialect.columnNames
  if (!columnNames) return records

  return records.map(row =>
    Object.fromEntries(columnNames.map(name => [name, row[name]])),
  )
}

function getJsonPointerValue(document: unknown, pointer: string) {
  let value = document
  for (const token of parseJsonPointer(pointer)) {
    if (Array.isArray(value)) {
      value = value[Number(token)]
    } else if (getIsObject(value) && Object.hasOwn(value, token)) {
      value = value[token]
    } else {
      return undefined
    }
  }

  return value
}
