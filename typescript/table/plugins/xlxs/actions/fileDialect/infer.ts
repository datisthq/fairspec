import type { OdsFileDialect, Resource, XlsxFileDialect } from "@fairspec/metadata"
import { getDataFirstPath, getSupportedFileDialect } from "@fairspec/metadata"
import { readSheetRows } from "../../helpers/sheet.ts"
import { formatSheetCell } from "../../helpers/table.ts"
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

  const rows = await readSheetRows(dataPath, fileDialect, { maxRows: sampleRows })
  if (!rows) {
    return { format: fileDialect.format }
  }

  if (rows.length === 0) {
    return { format: fileDialect.format }
  }

  const sniffer = new Sniffer()
  let detection: ReturnType<typeof sniffer.sniffRows>

  try {
    detection = sniffer.sniffRows(rows.map(row => row.map(formatSheetCell)))
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
