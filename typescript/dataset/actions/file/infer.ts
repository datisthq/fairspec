import { stat } from "node:fs/promises"
import type { Resource } from "@fairspec/metadata"
import { getDataPaths } from "@fairspec/metadata"
import * as hasha from "hasha"
import pMap from "p-map"
import { loadFile } from "../../actions/file/load.ts"
import { concatFileStreams } from "../../actions/stream/concat.ts"
import { loadFileStream } from "../../actions/stream/load.ts"
import { prefetchFiles } from "./prefetch.ts"

export type HashType = NonNullable<Resource["integrity"]>["type"]

export async function inferTextual(
  resource: Resource,
  options?: { sampleBytes?: number },
) {
  const maxBytes = options?.sampleBytes ?? 10_000

  const paths = getDataPaths(resource)
  if (!paths.length) {
    return false
  }

  for (const path of paths) {
    const isTextual = await getIsTextual(path, { maxBytes })
    if (!isTextual) {
      return false
    }
  }

  return true
}

export async function inferIntegrity(
  resource: Resource,
  options?: { hashType?: HashType },
) {
  const type = options?.hashType ?? "sha256"
  const hash = await inferHash(resource, { hashType: type })

  if (!hash) {
    return undefined
  }

  return { type, hash }
}

export async function inferHash(resource: Resource, options?: { hashType?: HashType }) {
  const hashType = options?.hashType ?? "sha256"
  const localPaths = await prefetchFiles(resource)

  if (!localPaths.length) {
    return ""
  }

  const streams = await pMap(localPaths, async path => loadFileStream(path))
  const stream = concatFileStreams(streams)

  const hash = await hasha.hash(stream, { algorithm: hashType })
  return hash
}

export async function inferBytes(resource: Resource) {
  const localPaths = await prefetchFiles(resource)

  let bytes = 0
  for (const localPath of localPaths) {
    const result = await stat(localPath)
    bytes += result.size
  }

  return bytes
}

async function getIsTextual(path: string, options: { maxBytes: number }) {
  const buffer = await loadFile(path, { maxBytes: options.maxBytes + 1 })
  const isCut = buffer.length > options.maxBytes
  const sample = isCut ? buffer.subarray(0, options.maxBytes) : buffer

  if (getIsBinary(sample)) {
    return false
  }

  // only a sample cut short may end inside a character; a whole file may not
  try {
    new TextDecoder("utf-8", { fatal: true }).decode(sample, { stream: isCut })
    return true
  } catch {
    return false
  }
}

function getIsBinary(data: Uint8Array) {
  return data.some(byte => byte <= 7 || (byte >= 14 && byte <= 31))
}
