import { buffer } from "node:stream/consumers"
import { loadFile, loadFileStream } from "@fairspec/dataset"
import type { JsonFileDialect, JsonlFileDialect, Resource } from "@fairspec/metadata"
import { getDataFirstPath, getSupportedFileDialect } from "@fairspec/metadata"
import { Sniffer } from "../../../../utils/sniffer/sniffer.ts"
import { decodeJsonBuffer } from "../buffer/decode.ts"

export async function inferJsonFileDialect(
  resource: Resource,
  options?: {
    sampleRows?: number
  },
): Promise<JsonFileDialect | JsonlFileDialect | undefined> {
  const dataPath = getDataFirstPath(resource)
  if (!dataPath) {
    return undefined
  }

  const dialect = await getSupportedFileDialect(resource, ["json", "jsonl"])
  if (!dialect) {
    return undefined
  }

  let jsonBuffer: Buffer
  try {
    if (dialect.format === "json") {
      jsonBuffer = await loadFile(dataPath)
    } else {
      const stream = await loadFileStream(dataPath, { maxBytes: 10000 })
      jsonBuffer = await buffer(stream)
    }
  } catch {
    return { format: dialect.format }
  }

  let parsed: unknown
  try {
    parsed = decodeJsonBuffer(jsonBuffer, {
      isLines: dialect.format === "jsonl",
    })
  } catch {
    return { format: dialect.format }
  }

  let jsonPointer: string | undefined
  let rowType: "array" | "object" | undefined
  let headerRows: number[] | false | undefined

  let data = parsed
  if (!Array.isArray(data)) {
    if (typeof data === "object" && data !== null) {
      for (const key of Object.keys(data)) {
        const value = (data as Record<string, unknown>)[key]
        if (Array.isArray(value)) {
          if (dialect.format === "json") {
            jsonPointer = key
          }
          data = value
          break
        }
      }
    }
  }

  if (!Array.isArray(data) || data.length === 0) {
    if (dialect.format === "json") {
      return { format: "json", jsonPointer, rowType, headerRows }
    }
    return { format: "jsonl", rowType, headerRows }
  }

  const firstElement = data[0]

  if (Array.isArray(firstElement)) {
    rowType = "array"
  } else if (typeof firstElement === "object" && firstElement !== null) {
    rowType = "object"
  } else {
    if (dialect.format === "json") {
      return { format: "json", jsonPointer, rowType, headerRows }
    }
    return { format: "jsonl", rowType, headerRows }
  }

  if (rowType === "array") {
    const sampleRows = options?.sampleRows ?? 100
    const rows = (data as unknown[][]).slice(0, sampleRows)

    const sniffer = new Sniffer()
    let detection: ReturnType<typeof sniffer.sniffRows>

    try {
      detection = sniffer.sniffRows(rows)
    } catch {
      if (dialect.format === "json") {
        return { format: "json", jsonPointer, rowType, headerRows }
      }
      return { format: "jsonl", rowType, headerRows }
    }

    if (detection.dialect.header.hasHeaderRow) {
      headerRows = [detection.dialect.header.numPreambleRows + 1]
    } else if (detection.numFields > 0) {
      headerRows = false
    }
  }

  if (dialect.format === "json") {
    return { format: "json", jsonPointer, rowType, headerRows }
  }
  return { format: "jsonl", rowType, headerRows }
}
