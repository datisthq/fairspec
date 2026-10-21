import { open } from "node:fs/promises"
import { Unzip, UnzipInflate } from "fflate"

export const MAX_SHEET_CELLS = 50_000_000

/**
 * Throws when the workbook's sheets span more cells than a load can hold. calamine
 * allocates every cell between the first and the last used one before returning,
 * so a tiny file naming a far cell would abort the process.
 */
export async function assertSheetSize(path: string) {
  let total = 0
  let error: unknown
  let isXlsb = false

  const checkCells = (cells: number) => {
    if (total + cells > MAX_SHEET_CELLS) {
      throw new Error(
        `Sheet is too large to load: it spans more than ${MAX_SHEET_CELLS} cells`,
      )
    }
  }

  const unzip = new Unzip(file => {
    if (file.name === "xl/workbook.bin") isXlsb = true

    const scanner = XLSX_SHEET.test(file.name)
      ? createXlsxScanner(checkCells)
      : file.name === "content.xml"
        ? createOdsScanner(checkCells)
        : undefined
    if (!scanner) return

    const decoder = new TextDecoder()
    file.ondata = (fileError, chunk, final) => {
      if (error) return
      try {
        if (fileError) throw fileError
        scanner.push(decoder.decode(chunk, { stream: !final }))
        if (final) total += scanner.finish()
      } catch (cause) {
        error = cause
      }
    }
    file.start()
  })
  unzip.register(UnzipInflate)

  await using handle = await open(path)
  const buffer = Buffer.alloc(1 << 20)
  let first = true
  while (!error) {
    const { bytesRead } = await handle.read(buffer, 0, buffer.length, null)
    if (first && !buffer.subarray(0, Math.min(bytesRead, 4)).equals(ZIP_MAGIC)) return
    first = false
    unzip.push(new Uint8Array(buffer.subarray(0, bytesRead)), bytesRead === 0)
    if (bytesRead === 0) break
  }

  if (error) throw error
  if (isXlsb) throw new Error("Spreadsheet format xlsb is not supported")
}

const ZIP_MAGIC = Buffer.from([0x50, 0x4b, 0x03, 0x04])
const XLSX_SHEET = /^xl\/worksheets\/[^/]+\.xml$/
const MAX_TAG_LENGTH = 1 << 24

// Quoted attribute values may hold ">", which must not end the tag early
const ATTRS = `([^>"']*(?:(?:"[^"]*"|'[^']*')[^>"']*)*)`
const XLSX_TAG = new RegExp(`<(?:[\\w.-]+:)?(c|row)(?=[\\s/>])${ATTRS}>`, "g")
const ODS_TAG = new RegExp(
  `<(\\/?)(?:[\\w.-]+:)?(table-row|table-cell|covered-table-cell|table)(?=[\\s/>])${ATTRS}>`,
  "g",
)
const ATTRIBUTES = new Map<string, RegExp>()

type CheckCells = (cells: number) => void

function createXlsxScanner(checkCells: CheckCells) {
  const bounds = createBounds(checkCells)
  let row = 0
  let col = 0

  const push = createTagReader(XLSX_TAG, match => {
    const attrs = match[2] ?? ""

    if (match[1] === "row") {
      row = toNumber(getAttribute(attrs, "r")) || row + 1
      col = 0
      return
    }

    const ref = /^([A-Za-z]+)(\d+)$/.exec(getAttribute(attrs, "r") ?? "")
    if (ref) {
      col = parseColumnLetters(ref[1] ?? "")
      row = Number(ref[2])
    } else {
      col += 1
    }

    if (!attrs.endsWith("/")) bounds.add(row, row, col, col)
  })

  return { push, finish: bounds.count }
}

function createOdsScanner(checkCells: CheckCells) {
  let previous = 0
  let bounds = createBounds(checkCells)
  let row = 0
  let col = 0
  let rowRepeat = 1
  let rowMinCol = Infinity
  let rowMaxCol = 0

  const push = createTagReader(ODS_TAG, match => {
    const isClosing = match[1] === "/"
    const name = match[2]
    const attrs = match[3] ?? ""
    const isEmpty = attrs.endsWith("/")

    if (name === "table" && !isClosing) {
      previous += bounds.count()
      bounds = createBounds(cells => checkCells(previous + cells))
      row = 0
      return
    }

    if (name === "table-row") {
      if (!isClosing) {
        rowRepeat = getRepeat(attrs, "number-rows-repeated")
        col = 0
      }
      if (isClosing || isEmpty) {
        if (rowMaxCol) bounds.add(row + 1, row + rowRepeat, rowMinCol, rowMaxCol)
        row += rowRepeat
        rowMinCol = Infinity
        rowMaxCol = 0
      }
      return
    }

    if (name !== "table" && !isClosing) {
      const start = col + 1
      col += getRepeat(attrs, "number-columns-repeated")
      if (!isEmpty || getAttribute(attrs, "value-type") !== undefined) {
        rowMinCol = Math.min(rowMinCol, start)
        rowMaxCol = Math.max(rowMaxCol, col)
      }
    }
  })

  return { push, finish: () => previous + bounds.count() }
}

function createBounds(checkCells: CheckCells) {
  let minRow = Infinity
  let maxRow = 0
  let minCol = Infinity
  let maxCol = 0

  const count = () => (maxRow ? (maxRow - minRow + 1) * (maxCol - minCol + 1) : 0)

  const add = (fromRow: number, toRow: number, fromCol: number, toCol: number) => {
    if (fromRow >= minRow && toRow <= maxRow && fromCol >= minCol && toCol <= maxCol)
      return
    minRow = Math.min(minRow, fromRow)
    maxRow = Math.max(maxRow, toRow)
    minCol = Math.min(minCol, fromCol)
    maxCol = Math.max(maxCol, toCol)
    checkCells(count())
  }

  return { add, count }
}

// Only the text after the last "<" can hold a tag that the next chunk completes
function createTagReader(pattern: RegExp, onTag: (match: RegExpExecArray) => void) {
  let rest = ""

  return (text: string) => {
    const input = rest + text
    let end = 0
    pattern.lastIndex = 0
    for (let match = pattern.exec(input); match; match = pattern.exec(input)) {
      onTag(match)
      end = pattern.lastIndex
    }

    const open = input.lastIndexOf("<")
    rest = open >= end ? input.slice(open) : ""
    if (rest.length > MAX_TAG_LENGTH) throw new Error("Spreadsheet markup is malformed")
  }
}

function getAttribute(attrs: string, name: string) {
  const quick = ` ${name}="`
  if (attrs.startsWith(quick))
    return attrs.slice(quick.length, attrs.indexOf('"', quick.length))

  let pattern = ATTRIBUTES.get(name)
  if (!pattern) {
    pattern = new RegExp(`(?:^|\\s)(?:[\\w.-]+:)?${name}\\s*=\\s*(?:"([^"]*)"|'([^']*)')`)
    ATTRIBUTES.set(name, pattern)
  }

  const match = pattern.exec(attrs)
  return match ? (match[1] ?? match[2]) : undefined
}

function getRepeat(attrs: string, name: string) {
  return Math.max(1, toNumber(getAttribute(attrs, name)) || 1)
}

function toNumber(value: string | undefined) {
  return value && /^\d+$/.test(value) ? Number(value) : 0
}

function parseColumnLetters(letters: string) {
  let col = 0
  for (const letter of letters.toUpperCase()) col = col * 26 + letter.charCodeAt(0) - 64
  return col
}
