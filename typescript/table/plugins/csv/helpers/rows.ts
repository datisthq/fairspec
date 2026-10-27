import { Buffer } from "node:buffer"
import { createReadStream, createWriteStream } from "node:fs"
import { Transform } from "node:stream"
import { pipeline } from "node:stream/promises"
import { getTempFilePath } from "@fairspec/dataset"
import type { CsvFileDialect, TsvFileDialect } from "@fairspec/metadata"
import { getHeaderRows } from "../../../helpers/fileDialect.ts"

const CR = 0x0d
const LF = 0x0a
const BOM = Buffer.from([0xef, 0xbb, 0xbf])
const LONG_ROW_MESSAGE = "found more fields than defined in the header"

/**
 * Polars reads a blank line as a row of nulls and a quoted `nullSequence` as null, so
 * files holding a blank line, or read with a `nullSequence`, are copied with blank lines
 * dropped and unquoted sequences emptied; the dialect returned has no `nullSequence`.
 * With several header rows, which nodejs-polars can only read truncating long rows, the
 * files are copied too, failing on a data row wider than every header row
 */
export async function cleanCsvRows(
  paths: string[],
  fileDialect: CsvFileDialect | TsvFileDialect,
  options?: { maxBytes?: number },
) {
  const end = options?.maxBytes ? options.maxBytes - 1 : undefined
  const eol = Buffer.from(fileDialect.lineTerminator ?? "\n").at(-1) ?? LF
  const nullSequence = fileDialect.nullSequence
  const headerRows = getHeaderRows(fileDialect)
  const isMultiHeader = headerRows.length > 1 && !fileDialect.columnNames

  if (nullSequence === undefined && !isMultiHeader) {
    let isBlank = false
    for (const path of paths) {
      if (await getHasBlankLine(path, eol, end)) {
        isBlank = true
        break
      }
    }
    if (!isBlank) return undefined
  }

  const scanner: RowScanner = {
    eol,
    delimiter:
      fileDialect.format === "csv"
        ? Buffer.from(fileDialect.delimiter ?? ",")[0]
        : Buffer.from("\t")[0],
    quote:
      fileDialect.format === "csv"
        ? Buffer.from(fileDialect.quoteChar ?? '"')[0]
        : undefined,
    nullSequence: nullSequence === undefined ? undefined : Buffer.from(nullSequence),
    commentPrefix: fileDialect.commentPrefix
      ? Buffer.from(fileDialect.commentPrefix)
      : undefined,
    lastHeaderRow: Math.max(0, ...headerRows),
    firstWidthRow: isMultiHeader ? Math.min(...headerRows) : undefined,
  }

  return {
    paths: await Promise.all(paths.map(path => cleanFileRows(path, scanner, end))),
    fileDialect: { ...fileDialect, nullSequence: undefined },
  }
}

interface RowScanner {
  eol: number
  delimiter?: number
  quote?: number
  nullSequence?: Buffer
  commentPrefix?: Buffer
  lastHeaderRow: number
  firstWidthRow?: number
}

// A blank line is two line ends with nothing (or only a CR) between them, or one at
// the start of the file after any BOM; the copy drops it unless inside a quoted cell
async function getHasBlankLine(path: string, eol: number, end?: number) {
  const needles = [Buffer.from([eol, eol])]
  if (eol === LF) needles.push(Buffer.from([LF, CR, LF]))

  let carry = Buffer.alloc(0)
  let isStart = true
  for await (const chunk of createReadStream(path, { end })) {
    if (!Buffer.isBuffer(chunk)) continue
    const data = Buffer.concat([carry, chunk])
    if (isStart) {
      const start = getBomLength(data)
      const first = data[start]
      if (first === eol || (eol === LF && first === CR && data[start + 1] === LF)) {
        return true
      }
      isStart = data.length < start + 2
    }
    if (needles.some(needle => data.includes(needle))) return true
    carry = data.subarray(-2)
  }

  return false
}

async function cleanFileRows(path: string, scanner: RowScanner, end?: number) {
  const parser = createRowParser(scanner)
  let isStart = true
  const transform = new Transform({
    transform(chunk: Buffer, _encoding, callback) {
      // The copy drops a BOM, so a quote or a line end right after it reads as at the start
      const start = isStart ? getBomLength(chunk) : 0
      isStart = false
      try {
        callback(null, parser.push(chunk.subarray(start)))
      } catch (error) {
        callback(error instanceof Error ? error : new Error(String(error)))
      }
    },
    flush(callback) {
      try {
        callback(null, parser.flush())
      } catch (error) {
        callback(error instanceof Error ? error : new Error(String(error)))
      }
    },
  })

  const target = getTempFilePath({ format: "csv" })
  await pipeline(createReadStream(path, { end }), transform, createWriteStream(target))
  return target
}

// A row is a record of the file: a line end inside a quoted cell does not end it. A
// quote opens a cell only at its start, and a doubled quote inside one is a quote
function createRowParser(scanner: RowScanner) {
  const { eol, nullSequence, commentPrefix, lastHeaderRow, firstWidthRow } = scanner

  let row: number[] = []
  let rowNumber = 0
  let cell = createCellState()
  let cellCount = 1
  let headerWidth = 0
  let isComment = false
  let isCommentCandidate = !!commentPrefix

  const emitRow = (output: Buffer[], isTerminated: boolean) => {
    const content = row.at(-1) === CR && eol === LF ? row.slice(0, -1) : row
    if (content.length) {
      rowNumber++
      const isData = rowNumber > lastHeaderRow && !isComment
      if (firstWidthRow !== undefined && !isComment) {
        if (rowNumber >= firstWidthRow && rowNumber <= lastHeaderRow) {
          headerWidth = Math.max(headerWidth, cellCount)
        } else if (isData && cellCount > headerWidth) {
          throw new Error(`${LONG_ROW_MESSAGE}: row ${rowNumber} has ${cellCount} cells`)
        }
      }
      const bytes =
        nullSequence && isData ? emptyNullCells(row, scanner) : Buffer.from(row)
      output.push(bytes)
      if (isTerminated) output.push(Buffer.from([eol]))
    }
    row = []
    cell = createCellState()
    cellCount = 1
    isComment = false
    isCommentCandidate = !!commentPrefix
  }

  return {
    push(chunk: Buffer) {
      const output: Buffer[] = []
      for (const byte of chunk) {
        if (byte === eol && !cell.isQuoted) {
          emitRow(output, true)
          continue
        }

        row.push(byte)
        if (commentPrefix && isCommentCandidate) {
          if (commentPrefix[row.length - 1] !== byte) isCommentCandidate = false
          else if (row.length === commentPrefix.length) isComment = true
        }
        if (!isComment && stepCell(cell, byte, scanner)) cellCount++
      }
      return Buffer.concat(output)
    },
    flush() {
      const output: Buffer[] = []
      if (row.length) emitRow(output, false)
      return Buffer.concat(output)
    },
  }
}

interface CellState {
  isQuoted: boolean
  isStart: boolean
  isClosed: boolean
}

function createCellState(): CellState {
  return { isQuoted: false, isStart: true, isClosed: false }
}

// Returns true when the byte ends the cell (an unquoted delimiter)
function stepCell(state: CellState, byte: number, scanner: RowScanner) {
  const { delimiter, quote } = scanner
  if (state.isQuoted) {
    if (byte === quote) {
      state.isQuoted = false
      state.isClosed = true
    }
    return false
  }
  if (byte === delimiter) {
    Object.assign(state, createCellState())
    return true
  }
  if (quote !== undefined && byte === quote && (state.isStart || state.isClosed)) {
    state.isQuoted = true
  }
  state.isStart = false
  state.isClosed = false
  return false
}

function emptyNullCells(bytes: number[], scanner: RowScanner) {
  const { eol, delimiter, nullSequence } = scanner
  const cells: number[][] = [[]]
  const state = createCellState()
  for (const byte of bytes) {
    if (stepCell(state, byte, scanner)) cells.push([])
    else cells.at(-1)?.push(byte)
  }

  const parts = cells.map(cell => {
    const text = cell.at(-1) === CR && eol === LF ? cell.slice(0, -1) : cell
    return nullSequence?.equals(Buffer.from(text)) ? cell.slice(text.length) : cell
  })
  return Buffer.from(
    parts.flatMap((part, index) => (index ? [delimiter ?? 0, ...part] : part)),
  )
}

function getBomLength(data: Buffer) {
  return data.subarray(0, BOM.length).equals(BOM) ? BOM.length : 0
}
