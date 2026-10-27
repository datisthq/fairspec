import { Buffer } from "node:buffer"
import { createReadStream, createWriteStream } from "node:fs"
import { Transform } from "node:stream"
import { pipeline } from "node:stream/promises"
import { getTempFilePath } from "@fairspec/dataset"
import type { CsvFileDialect, TsvFileDialect } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import type { Table } from "../../../models/table.ts"

// Control bytes a text file almost never holds; tab, LF and CR are left out
const PLACEHOLDER_BYTES = [
  31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 12, 11, 8, 7, 6,
  5, 4, 3, 2, 1,
]

export interface DelimiterSwap {
  delimiter: string
  placeholder: string
}

/**
 * Polars splits cells on one byte, so a multi-byte delimiter is swapped in copies of the
 * files for a control byte none of them holds; `restoreDelimiter` swaps it back
 */
export async function swapDelimiter(
  paths: string[],
  fileDialect: CsvFileDialect | TsvFileDialect,
  options?: { maxBytes?: number },
) {
  if (fileDialect.format !== "csv") return undefined
  const { delimiter, quoteChar, lineTerminator, nullSequence, commentPrefix } =
    fileDialect
  if (!delimiter || Buffer.byteLength(delimiter) === 1) return undefined

  const reserved = Buffer.from(
    [quoteChar, lineTerminator, nullSequence, commentPrefix].join(""),
  )
  // A preview reads only its first bytes, so neither pass reads past them
  const end = options?.maxBytes ? options.maxBytes - 1 : undefined
  const placeholder = await findAbsentByte(
    paths,
    PLACEHOLDER_BYTES.filter(byte => !reserved.includes(byte)),
    end,
  )
  if (placeholder === undefined) {
    throw new Error(
      `Delimiter "${delimiter}" cannot be read: the file holds every control byte`,
    )
  }

  const swap = { delimiter, placeholder: String.fromCharCode(placeholder) }
  const swapText = (text?: string) => text?.replaceAll(delimiter, swap.placeholder)
  return {
    swap,
    paths: await Promise.all(
      paths.map(path => swapFileDelimiter(path, delimiter, placeholder, end)),
    ),
    fileDialect: {
      ...fileDialect,
      delimiter: swap.placeholder,
      nullSequence: swapText(nullSequence),
      commentPrefix: swapText(commentPrefix),
    },
  }
}

/** Puts the multi-byte delimiter back in every cell and column name */
export function restoreDelimiter(table: Table, swap: DelimiterSwap) {
  const { delimiter, placeholder } = swap
  return table.select(
    table.columns.map(name =>
      pl
        .col(name)
        .str.replaceAll(placeholder, delimiter, true)
        .alias(name.replaceAll(placeholder, delimiter)),
    ),
  )
}

async function findAbsentByte(paths: string[], candidates: number[], end?: number) {
  for (const byte of candidates) {
    let isPresent = false
    for (const path of paths) {
      for await (const chunk of createReadStream(path, { end })) {
        if (Buffer.isBuffer(chunk) && chunk.includes(byte)) {
          isPresent = true
          break
        }
      }
      if (isPresent) break
    }
    if (!isPresent) return byte
  }
  return undefined
}

async function swapFileDelimiter(
  path: string,
  delimiter: string,
  placeholder: number,
  end?: number,
) {
  const needle = Buffer.from(delimiter)
  const replacement = Buffer.from([placeholder])
  let carry = Buffer.alloc(0)

  const transform = new Transform({
    transform(chunk: Buffer, _encoding, callback) {
      const data = Buffer.concat([carry, chunk])
      // A delimiter cut by the chunk boundary waits for the next chunk
      let cut = data.length
      for (let size = needle.length - 1; size > 0; size--) {
        if (data.subarray(-size).equals(needle.subarray(0, size))) {
          cut = data.length - size
          break
        }
      }
      carry = data.subarray(cut)
      callback(null, replaceBytes(data.subarray(0, cut), needle, replacement))
    },
    flush(callback) {
      callback(null, carry)
    },
  })

  const target = getTempFilePath({ format: "csv" })
  await pipeline(createReadStream(path, { end }), transform, createWriteStream(target))
  return target
}

function replaceBytes(data: Buffer, needle: Buffer, replacement: Buffer) {
  const parts: Buffer[] = []
  let start = 0
  let index = data.indexOf(needle)
  while (index !== -1) {
    parts.push(data.subarray(start, index), replacement)
    start = index + needle.length
    index = data.indexOf(needle, start)
  }
  parts.push(data.subarray(start))
  return Buffer.concat(parts)
}
