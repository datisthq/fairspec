import { createReadStream } from "node:fs"
import { open } from "node:fs/promises"
import { createInflateRaw } from "node:zlib"

export type ZipEntry = {
  name: string
  method: number
  compressedSize: number
  headerOffset: number
}

/**
 * Lists a zip archive's entries from its central directory, as zip readers do, or
 * returns undefined when the file is not a zip archive.
 */
export async function readZipEntries(path: string) {
  await using handle = await open(path)
  const { size } = await handle.stat()

  const tailLength = Math.min(size, MAX_TAIL_LENGTH)
  const tail = Buffer.alloc(tailLength)
  await handle.read(tail, 0, tailLength, size - tailLength)

  const end = tail.lastIndexOf(EOCD_SIGNATURE)
  if (end === -1) return undefined
  const endOffset = size - tailLength + end

  let count = tail.readUInt16LE(end + 10)
  let directorySize = tail.readUInt32LE(end + 12)
  let directoryOffset = tail.readUInt32LE(end + 16)
  let directoryStart = endOffset - directorySize

  if (
    count === 0xffff ||
    directorySize === 0xffffffff ||
    directoryOffset === 0xffffffff
  ) {
    const locator = tail.lastIndexOf(ZIP64_LOCATOR_SIGNATURE, end)
    if (locator === -1) throw new Error("Spreadsheet archive is malformed")
    const record = Buffer.alloc(56)
    const recordStart = size - tailLength + locator - record.length
    await handle.read(record, 0, record.length, recordStart)
    if (!record.subarray(0, 4).equals(ZIP64_END_SIGNATURE)) {
      throw new Error("Spreadsheet archive is malformed")
    }
    count = Number(record.readBigUInt64LE(32))
    directorySize = Number(record.readBigUInt64LE(40))
    directoryOffset = Number(record.readBigUInt64LE(48))
    directoryStart = recordStart - directorySize
  }

  // Data prepended to the archive shifts every offset, and zip readers allow it
  const shift = directoryStart - directoryOffset
  if (directoryStart < 0 || shift < 0) throw new Error("Spreadsheet archive is malformed")

  const directory = Buffer.alloc(directorySize)
  await handle.read(directory, 0, directorySize, directoryStart)

  const entries: ZipEntry[] = []
  let position = 0
  for (let index = 0; index < count; index++) {
    if (!directory.subarray(position, position + 4).equals(ENTRY_SIGNATURE)) {
      throw new Error("Spreadsheet archive is malformed")
    }

    const nameLength = directory.readUInt16LE(position + 28)
    const extraLength = directory.readUInt16LE(position + 30)
    const commentLength = directory.readUInt16LE(position + 32)
    const nameStart = position + 46
    const extraStart = nameStart + nameLength

    let compressedSize = directory.readUInt32LE(position + 20)
    let headerOffset = directory.readUInt32LE(position + 42)
    const zip64 = readZip64Extra(
      directory.subarray(extraStart, extraStart + extraLength),
      directory.readUInt32LE(position + 24) === 0xffffffff,
      compressedSize === 0xffffffff,
      headerOffset === 0xffffffff,
    )
    compressedSize = zip64.compressedSize ?? compressedSize
    headerOffset = zip64.headerOffset ?? headerOffset

    entries.push({
      name: directory.toString("utf8", nameStart, extraStart),
      method: directory.readUInt16LE(position + 10),
      compressedSize,
      headerOffset: headerOffset + shift,
    })
    position = extraStart + extraLength + commentLength
  }

  return entries
}

/**
 * Streams a zip entry's uncompressed bytes.
 */
export async function* readZipEntry(path: string, entry: ZipEntry) {
  if (entry.method !== 0 && entry.method !== 8) {
    throw new Error("Spreadsheet archive uses an unsupported compression method")
  }

  const header = Buffer.alloc(30)
  {
    await using handle = await open(path)
    await handle.read(header, 0, header.length, entry.headerOffset)
  }
  if (!header.subarray(0, 4).equals(LOCAL_SIGNATURE)) {
    throw new Error("Spreadsheet archive is malformed")
  }

  if (!entry.compressedSize) return
  const start =
    entry.headerOffset + 30 + header.readUInt16LE(26) + header.readUInt16LE(28)
  const stream = createReadStream(path, { start, end: start + entry.compressedSize - 1 })

  for await (const chunk of entry.method === 8
    ? stream.pipe(createInflateRaw())
    : stream) {
    if (Buffer.isBuffer(chunk)) yield chunk
  }
}

const MAX_TAIL_LENGTH = 22 + 0xffff
const EOCD_SIGNATURE = Buffer.from([0x50, 0x4b, 0x05, 0x06])
const ZIP64_LOCATOR_SIGNATURE = Buffer.from([0x50, 0x4b, 0x06, 0x07])
const ZIP64_END_SIGNATURE = Buffer.from([0x50, 0x4b, 0x06, 0x06])
const ENTRY_SIGNATURE = Buffer.from([0x50, 0x4b, 0x01, 0x02])
const LOCAL_SIGNATURE = Buffer.from([0x50, 0x4b, 0x03, 0x04])

function readZip64Extra(
  extra: Buffer,
  hasSize: boolean,
  hasCompressedSize: boolean,
  hasOffset: boolean,
) {
  const result: { compressedSize?: number; headerOffset?: number } = {}

  for (let position = 0; position + 4 <= extra.length;) {
    const id = extra.readUInt16LE(position)
    const length = extra.readUInt16LE(position + 2)
    if (id === 0x0001) {
      let field = position + 4
      if (hasSize) field += 8
      if (hasCompressedSize) {
        result.compressedSize = Number(extra.readBigUInt64LE(field))
        field += 8
      }
      if (hasOffset) result.headerOffset = Number(extra.readBigUInt64LE(field))
    }
    position += 4 + length
  }

  return result
}
