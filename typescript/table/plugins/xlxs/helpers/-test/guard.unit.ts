import { writeTempFile } from "@fairspec/dataset"
import { strToU8, zipSync } from "fflate"
import { describe, expect, it } from "vite-plus/test"
import { assertSheetSize } from "../guard.ts"

const TOO_LARGE = /Sheet is too large to load/

describe("assertSheetSize", () => {
  it("should accept a small sheet", async () => {
    const path = await writeXlsx(
      `<row r="1"><c r="A1"><v>1</v></c><c r="B1"><v>2</v></c></row>`,
    )
    await expect(assertSheetSize(path)).resolves.toBeUndefined()
  })

  it("should reject a sheet naming a far cell", async () => {
    const path = await writeXlsx(
      `<row r="1"><c r="A1"><v>1</v></c></row>` +
        `<row r="1048576"><c r="XFD1048576"><v>1</v></c></row>`,
    )
    await expect(assertSheetSize(path)).rejects.toThrow(TOO_LARGE)
  })

  it("should accept a far cell without a value", async () => {
    const path = await writeXlsx(
      `<row r="1"><c r="A1"><v>1</v></c></row>` +
        `<row r="1048576"><c r="XFD1048576" s="1"/></row>`,
    )
    await expect(assertSheetSize(path)).resolves.toBeUndefined()
  })

  it("should read a reference after a quoted >", async () => {
    const path = await writeXlsx(
      `<row r="1"><c r="A1"><v>1</v></c></row>` +
        `<row><c t="n" x=">" r="XFD1048576"><v>1</v></c></row>`,
    )
    await expect(assertSheetSize(path)).rejects.toThrow(TOO_LARGE)
  })

  it("should place cells without a reference one after another", async () => {
    const wide = "<c><v>1</v></c>".repeat(10_001)
    const path = await writeXlsx(
      `<row r="1">${wide}</row><row r="5001"><c r="A5001"><v>1</v></c></row>`,
    )
    await expect(assertSheetSize(path)).rejects.toThrow(TOO_LARGE)
  })

  it("should reject ods cells repeated over a large range", async () => {
    const path = await writeOds(
      `<table:table-row table:number-rows-repeated="1048576">` +
        `<table:table-cell table:number-columns-repeated="16384" office:value-type="float" office:value="1"><text:p>1</text:p></table:table-cell>` +
        `</table:table-row>`,
    )
    await expect(assertSheetSize(path)).rejects.toThrow(TOO_LARGE)
  })

  it("should accept empty ods cells repeated to the sheet's edge", async () => {
    const path = await writeOds(
      `<table:table-row><table:table-cell office:value-type="float" office:value="1"><text:p>1</text:p></table:table-cell>` +
        `<table:table-cell table:number-columns-repeated="16383"/></table:table-row>` +
        `<table:table-row table:number-rows-repeated="1048575"><table:table-cell table:number-columns-repeated="16384"/></table:table-row>`,
    )
    await expect(assertSheetSize(path)).resolves.toBeUndefined()
  })

  it("should reject an xlsb workbook", async () => {
    const path = await writeTempFile(
      Buffer.from(zipSync({ "xl/workbook.bin": strToU8("") })),
    )
    await expect(assertSheetSize(path)).rejects.toThrow("xlsb is not supported")
  })

  it("should skip a file that is not a zip archive", async () => {
    const path = await writeTempFile("id,name\n1,a\n")
    await expect(assertSheetSize(path)).resolves.toBeUndefined()
  })
})

async function writeXlsx(sheetData: string) {
  const sheet = `<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>${sheetData}</sheetData></worksheet>`
  return await writeTempFile(
    Buffer.from(zipSync({ "xl/worksheets/sheet1.xml": strToU8(sheet) })),
  )
}

async function writeOds(rows: string) {
  const content = `<?xml version="1.0" encoding="UTF-8"?><office:document-content xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0" xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0"><office:body><office:spreadsheet><table:table table:name="Sheet1">${rows}</table:table></office:spreadsheet></office:body></office:document-content>`
  return await writeTempFile(Buffer.from(zipSync({ "content.xml": strToU8(content) })))
}
