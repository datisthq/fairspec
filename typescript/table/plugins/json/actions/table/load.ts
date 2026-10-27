import type { Resource } from "@fairspec/metadata"
import type { JsonFileDialect, JsonlFileDialect } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { loadFile, loadFileStream, prefetchFiles } from "@fairspec/dataset"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import { getRecordsFromRows } from "../../../../actions/data/fileDialect.ts"
import { getIsObject } from "../../../../helpers/general.ts"
import type { Table } from "../../../../models/table.ts"
import { Buffer } from "node:buffer"
import { buffer as readStreamBuffer } from "node:stream/consumers"
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

  // A declared dialect is binding: its absent properties take their defaults
  if (!resource.fileDialect) {
    fileDialect = await inferJsonFileDialect({ ...resource, data: paths[0] })
  }
  if (!fileDialect) {
    throw new Error("Resource data is not compatible")
  }

  const isLines = fileDialect.format === "jsonl"
  const isDefault = Object.entries(fileDialect).every(
    ([key, value]) =>
      ["format", "type", "title", "description"].includes(key) ||
      value === undefined ||
      (key === "rowType" && value === "object"),
  )

  const tables: Table[] = []
  for (const path of paths) {
    // Polars reads no byte order mark, and orders the keys of uneven rows by its own rule
    const isNative = isDefault && !(await getHasByteOrderMark(path))
    if (isLines && isNative && !(await getIsFirstItemArray(path))) {
      const keys = await readFirstLineKeys(path)
      const lineScan = pl.scanJson(path, { inferSchemaLength: null })
      const lineTable = keys && orderFirstSeenColumns(lineScan, keys)
      if (lineTable) {
        tables.push(lineTable)
        continue
      }
    }

    const buffer = await loadFile(path)
    const frame = !isLines && isNative ? readObjectArray(buffer) : undefined
    const frameKeys = frame && readFirstObjectKeys(buffer)
    const frameTable =
      frame && frameKeys && orderFirstSeenColumns(frame.lazy(), frameKeys)
    if (frameTable) {
      tables.push(frameTable)
      continue
    }

    const records = getRecordsFromJson(decodeJsonBuffer(buffer, { isLines }), fileDialect)
    if (!records) return undefined

    const table = createRecordsFrame(records, fileDialect.columnNames).lazy()
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

// An empty array has no records to take the columns from
// Polars takes a key first seen after the rows it infers from as no column at all
function createRecordsFrame(records: Record<string, unknown>[], columnNames?: string[]) {
  if (records.length || !columnNames) {
    return pl.DataFrame(records, { inferSchemaLength: records.length })
  }
  return pl.DataFrame(columnNames.map(name => pl.Series(name, [], pl.String)))
}

// The union of the keys is the first row's keys in their order unless a later row
// adds one, whose place then needs every row read in order
function orderFirstSeenColumns(table: Table, keys: string[]) {
  const columns = table.columns
  if (columns.length !== keys.length) return undefined
  if (!columns.every(name => keys.includes(name))) return undefined
  return table.select(keys)
}

async function getHasByteOrderMark(path: string) {
  const stream = await loadFileStream(path, { maxBytes: 3 })
  const start = await readStreamBuffer(stream)
  return start[0] === 0xef && start[1] === 0xbb && start[2] === 0xbf
}

async function readFirstLineKeys(path: string) {
  const stream = await loadFileStream(path)
  const chunks: Buffer[] = []
  try {
    for await (const chunk of stream) {
      const end = chunk.indexOf(0x0a)
      chunks.push(end === -1 ? chunk : chunk.subarray(0, end))
      if (end !== -1) break
    }
  } finally {
    stream.destroy()
  }
  return getObjectKeys(Buffer.concat(chunks).toString("utf-8"))
}

function readFirstObjectKeys(data: Buffer) {
  const start = data.indexOf(0x7b)
  let depth = 0
  let isString = false
  for (let index = start; index >= 0 && index < data.length; index++) {
    const byte = data[index]
    if (isString) {
      if (byte === 0x5c) index++
      else if (byte === 0x22) isString = false
    } else if (byte === 0x22) isString = true
    else if (byte === 0x7b || byte === 0x5b) depth++
    else if ((byte === 0x7d || byte === 0x5d) && --depth === 0) {
      return getObjectKeys(data.subarray(start, index + 1).toString("utf-8"))
    }
  }
  return undefined
}

function getObjectKeys(text: string) {
  try {
    const value: unknown = JSON.parse(text)
    return getIsObject(value) ? Object.keys(value) : undefined
  } catch {
    return undefined
  }
}

// Polars reads only object lines, while a default dialect takes array rows when
// the first item is an array
async function getIsFirstItemArray(path: string) {
  const stream = await loadFileStream(path)
  try {
    for await (const chunk of stream) {
      const index = skipJsonWhitespace(chunk, 0)
      if (index < chunk.length) return chunk[index] === 0x5b
    }
    return false
  } finally {
    stream.destroy()
  }
}

// A default dialect still admits a top-level object (not a table) or array rows,
// so only a document opening with `[{` is read natively, and a refusal falls back
function readObjectArray(buffer: Buffer) {
  const start = skipJsonWhitespace(buffer, 0)
  if (buffer[start] !== 0x5b) return undefined
  if (buffer[skipJsonWhitespace(buffer, start + 1)] !== 0x7b) return undefined

  try {
    return pl.readJSON(buffer, { inferSchemaLength: null })
  } catch {
    return undefined
  }
}

function skipJsonWhitespace(buffer: Buffer, index: number) {
  while ([0x20, 0x09, 0x0a, 0x0d].includes(buffer[index] ?? -1)) index++
  return index
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
