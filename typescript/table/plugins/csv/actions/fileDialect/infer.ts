import { buffer } from "node:stream/consumers"
import { loadFileStream } from "@fairspec/dataset"
import type { CsvFileDialect, Resource, TsvFileDialect } from "@fairspec/metadata"
import { getDataPath } from "@fairspec/metadata"
import { Sniffer } from "../../../../utils/sniffer/sniffer.ts"

export async function inferCsvFileDialect(
  resource: Resource,
  options?: {
    sampleBytes?: number
  },
) {
  const { sampleBytes = 10_000 } = options ?? {}

  const dataPath = getDataPath(resource)
  if (!dataPath) {
    return undefined
  }

  const stream = await loadFileStream(dataPath, {
    maxBytes: sampleBytes,
  })

  const sampleBuffer = await buffer(stream)
  const bytes = new Uint8Array(sampleBuffer)

  const sniffer = new Sniffer()
  let sniffResult: ReturnType<typeof sniffer.sniffBytes>

  try {
    sniffResult = sniffer.sniffBytes(bytes)
  } catch {
    const fallback: CsvFileDialect = { format: "csv" }
    return fallback
  }

  if (!sniffResult) {
    const fallback: CsvFileDialect = { format: "csv" }
    return fallback
  }

  const lt = sniffResult.dialect.lineTerminator
  const lineTerminator = lt === "LF" ? "\n" : lt === "CRLF" ? "\r\n" : "\r"

  const format = sniffResult.dialect.delimiter === 9 ? "tsv" : "csv"
  let dialect: CsvFileDialect | TsvFileDialect

  if (format === "csv") {
    const csvDialect: CsvFileDialect = {
      format: "csv",
      delimiter: String.fromCharCode(sniffResult.dialect.delimiter),
      lineTerminator,
    }

    if (sniffResult.dialect.quote.type === "Some") {
      csvDialect.quoteChar = String.fromCharCode(sniffResult.dialect.quote.char)
    }

    if (sniffResult.dialect.header.hasHeaderRow) {
      csvDialect.headerRows = [sniffResult.dialect.header.numPreambleRows + 1]
    } else if (sniffResult.numFields > 0) {
      csvDialect.headerRows = false
    }

    dialect = csvDialect
  } else {
    const tsvDialect: TsvFileDialect = {
      format: "tsv",
      lineTerminator,
    }

    if (sniffResult.dialect.header.hasHeaderRow) {
      tsvDialect.headerRows = [sniffResult.dialect.header.numPreambleRows + 1]
    } else if (sniffResult.numFields > 0) {
      tsvDialect.headerRows = false
    }

    dialect = tsvDialect
  }

  return dialect
}
