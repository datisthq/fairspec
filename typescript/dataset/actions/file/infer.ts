import { stat } from "node:fs/promises"
import type { Resource } from "@fairspec/metadata"
import { getDataFirstPath } from "@fairspec/metadata"
import chardet from "chardet"
import * as hasha from "hasha"
import { isBinaryFile } from "isbinaryfile"
import pMap from "p-map"
import { loadFile } from "../../actions/file/load.ts"
import { concatFileStreams } from "../../actions/stream/concat.ts"
import { loadFileStream } from "../../actions/stream/load.ts"
import { prefetchFiles } from "./prefetch.ts"

export type HashType = NonNullable<Resource["integrity"]>["type"]

export async function inferTextual(
  resource: Resource,
  options?: { sampleBytes?: number; confidencePercent?: number },
) {
  const maxBytes = options?.sampleBytes ?? 10_000
  const confidencePercent = options?.confidencePercent ?? 80

  const firstPath = getDataFirstPath(resource)
  if (!firstPath) {
    return false
  }

  const buffer = await loadFile(firstPath, { maxBytes })
  const isBinary = await isBinaryFile(buffer)
  if (isBinary) {
    return false
  }

  const matches = chardet.analyse(buffer)
  for (const match of matches) {
    if (match.confidence >= confidencePercent) {
      const encoding = match.name.toLowerCase()
      return ["utf-8", "ascii"].includes(encoding)
    }
  }

  return false
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
