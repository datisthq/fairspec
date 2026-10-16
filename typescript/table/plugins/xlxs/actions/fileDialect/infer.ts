import { loadFile } from "@fairspec/dataset"
import type { OdsFileDialect, Resource, XlsxFileDialect } from "@fairspec/metadata"
import { getDataFirstPath, getSupportedFileDialect } from "@fairspec/metadata"
import { read, utils } from "xlsx"
import { Sniffer } from "../../../../utils/sniffer/sniffer.ts"

export async function inferXlsxFileDialect(
  resource: Resource,
  options?: {
    sampleRows?: number
  },
): Promise<XlsxFileDialect | OdsFileDialect | undefined> {
  const { sampleRows = 100 } = options ?? {}

  const dataPath = getDataFirstPath(resource)
  if (!dataPath) {
    return undefined
  }

  const fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
  if (!fileDialect) {
    return undefined
  }

  const buffer = await loadFile(dataPath)
  const book = read(buffer, { type: "buffer" })

  const sheetIndex = fileDialect?.sheetNumber ? fileDialect.sheetNumber - 1 : 0
  const sheetName = fileDialect?.sheetName ?? book.SheetNames[sheetIndex]

  if (!sheetName) {
    return { format: fileDialect.format }
  }

  const sheet = book.Sheets[sheetName]
  if (!sheet) {
    return { format: fileDialect.format }
  }

  const allRows = utils.sheet_to_json(sheet, {
    header: 1,
    raw: true,
    defval: null,
  }) as unknown[][]

  const rows = allRows.slice(0, sampleRows)

  if (rows.length === 0) {
    return { format: fileDialect.format }
  }

  const sniffer = new Sniffer()
  let detection: ReturnType<typeof sniffer.sniffRows>

  try {
    detection = sniffer.sniffRows(rows)
  } catch {
    return { format: fileDialect.format }
  }

  const result = { format: fileDialect.format } as XlsxFileDialect | OdsFileDialect

  if (detection.dialect.header.hasHeaderRow) {
    result.headerRows = [detection.dialect.header.numPreambleRows + 1]
  } else if (detection.numFields > 0) {
    result.headerRows = false
  }

  return result
}
