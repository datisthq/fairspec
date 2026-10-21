import { writeFile } from "node:fs/promises"
import { createReader } from "calamine-node"
import type { SheetValue } from "../../../helpers/write.ts"
import { writeOdsBuffer, writeXlsxBuffer } from "../../../helpers/write.ts"

// We intentionally don't use fairspec's loader here to isolate the tests

export async function readTestData(path: string) {
  const reader = createReader({ experimentalFormats: ["ods"] })
  await using book = await reader.openFile(path)
  const [header = [], ...rows] = (await book.readSheet(0)).rows

  return rows.map(row =>
    Object.fromEntries(header.map((label, index) => [String(label), row[index]])),
  )
}

export async function writeTestData(
  path: string,
  rows: SheetValue[][],
  options?: {
    sheetNumber?: number
    sheetName?: string
    format?: "xlsx" | "ods"
  },
) {
  const sheetNumber = options?.sheetNumber ?? 1
  const sheets = Array.from({ length: sheetNumber }, (_, index) => ({
    name: options?.sheetName ?? `Sheet${index + 1}`,
    rows,
  }))

  const buffer =
    options?.format === "ods" ? writeOdsBuffer(sheets) : writeXlsxBuffer(sheets)
  await writeFile(path, buffer)
}
