import { strToU8, zipSync } from "fflate"
import { Workbook } from "wasm-xlsxwriter"

export type SheetValue = string | number | boolean | null
export type SheetData = { name: string; rows: SheetValue[][] }

/**
 * Writes sheets as an xlsx workbook.
 */
export function writeXlsxBuffer(sheets: SheetData[]) {
  const book = new Workbook()

  for (const { name, rows } of sheets) {
    const sheet = book.addWorksheet().setName(name)
    for (const [row, values] of rows.entries()) {
      for (const [col, value] of values.entries()) {
        if (typeof value === "number") sheet.writeNumber(row, col, value)
        else if (typeof value === "boolean") sheet.writeBoolean(row, col, value)
        else if (typeof value === "string") sheet.writeString(row, col, value)
      }
    }
  }

  return toBuffer(book.saveToBufferSync())
}

/**
 * Writes sheets as an ods document.
 */
export function writeOdsBuffer(sheets: SheetData[]) {
  const tables = sheets.map(({ name, rows }) => {
    const table = rows
      .map(row => `<table:table-row>${row.map(createOdsCell).join("")}</table:table-row>`)
      .join("")
    return `<table:table table:name="${escapeXml(name)}">${table}</table:table>`
  })

  const content =
    `<?xml version="1.0" encoding="UTF-8"?>` +
    `<office:document-content xmlns:office="${OFFICE_NS}" xmlns:table="${TABLE_NS}" ` +
    `xmlns:text="${TEXT_NS}" office:version="1.2"><office:body><office:spreadsheet>` +
    `${tables.join("")}</office:spreadsheet></office:body></office:document-content>`

  return toBuffer(
    zipSync({
      mimetype: [strToU8(ODS_MIMETYPE), { level: 0 }],
      "META-INF/manifest.xml": strToU8(ODS_MANIFEST),
      "content.xml": strToU8(content),
    }),
  )
}

const OFFICE_NS = "urn:oasis:names:tc:opendocument:xmlns:office:1.0"
const TABLE_NS = "urn:oasis:names:tc:opendocument:xmlns:table:1.0"
const TEXT_NS = "urn:oasis:names:tc:opendocument:xmlns:text:1.0"
const ODS_MIMETYPE = "application/vnd.oasis.opendocument.spreadsheet"
const ODS_MANIFEST =
  `<?xml version="1.0" encoding="UTF-8"?>` +
  `<manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" ` +
  `manifest:version="1.2">` +
  `<manifest:file-entry manifest:full-path="/" manifest:media-type="${ODS_MIMETYPE}"/>` +
  `<manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml"/>` +
  `</manifest:manifest>`

// calamine rejects whitespace between cells, so the document is written without any
function createOdsCell(value: SheetValue) {
  if (value === null) return "<table:table-cell/>"

  if (typeof value === "number") {
    const text = String(value)
    return `<table:table-cell office:value-type="float" office:value="${text}"><text:p>${text}</text:p></table:table-cell>`
  }

  if (typeof value === "boolean") {
    const text = String(value)
    return `<table:table-cell office:value-type="boolean" office:boolean-value="${text}"><text:p>${text.toUpperCase()}</text:p></table:table-cell>`
  }

  return `<table:table-cell office:value-type="string"><text:p>${escapeXml(value)}</text:p></table:table-cell>`
}

function escapeXml(text: string) {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
}

function toBuffer(bytes: Uint8Array) {
  return Buffer.from(bytes.buffer, bytes.byteOffset, bytes.byteLength)
}
