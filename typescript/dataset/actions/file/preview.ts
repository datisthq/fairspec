import { isBinaryFile } from "isbinaryfile"
import { loadFile } from "./load.ts"

/**
 * Read the raw source of a local or remote file as UTF-8 text, up to the first
 * `maxLines` lines (default 20) or `maxBytes` bytes (default 10,000), whichever comes first
 */
export async function previewFile(
  path: string,
  options?: { maxBytes?: number; maxLines?: number },
) {
  const maxBytes = options?.maxBytes ?? 10_000
  const maxLines = options?.maxLines ?? 20
  assertPositiveInteger("maxBytes", maxBytes)
  assertPositiveInteger("maxLines", maxLines)

  // one extra byte tells a file of exactly maxBytes apart from a longer one
  const buffer = await loadFile(path, { maxBytes: maxBytes + 1 })
  let truncated = buffer.length > maxBytes
  let sample = buffer.subarray(0, maxBytes)
  if (truncated) {
    sample = sample.subarray(0, getCompleteUtf8Length(sample))
  }

  if (await isBinaryFile(sample)) {
    throw new Error("Cannot preview a binary file")
  }

  let text = new TextDecoder("utf-8").decode(sample)

  let end = -1
  for (let line = 0; line < maxLines; line++) {
    end = text.indexOf("\n", end + 1)
    if (end === -1) break
  }

  if (end !== -1 && end + 1 < text.length) {
    text = text.slice(0, end + 1)
    truncated = true
  }

  return { text, truncated }
}

// isBinaryFile reads a multi-byte character cut by the byte limit as binary in a small sample
function getCompleteUtf8Length(buffer: Uint8Array) {
  for (let index = buffer.length - 1; index >= Math.max(0, buffer.length - 4); index--) {
    const byte = buffer[index] ?? 0
    if ((byte & 0xc0) === 0x80) continue
    if (byte < 0xc0) return buffer.length

    const size = byte >= 0xf0 ? 4 : byte >= 0xe0 ? 3 : 2
    return buffer.length - index < size ? index : buffer.length
  }

  return buffer.length
}

function assertPositiveInteger(name: string, value: number) {
  if (!Number.isInteger(value) || value <= 0) {
    throw new Error(`${name} must be a positive integer`)
  }
}
