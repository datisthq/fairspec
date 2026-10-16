import { createWriteStream } from "node:fs"
import { mkdir } from "node:fs/promises"
import nodePath from "node:path"
import { Readable, Transform } from "node:stream"
import { pipeline } from "node:stream/promises"
import { getFileExtension, getIsRemotePath } from "@fairspec/library"
import type { FileType } from "#models/file.ts"
import * as settings from "#settings.ts"

export async function prefetchFile(
  source: string | File,
  options: {
    folder: string
    fileType: FileType
  },
) {
  const { folder, fileType } = options

  if (!source) {
    return undefined
  }

  if (typeof source === "string") {
    const isRemote = getIsRemotePath(source)
    if (!isRemote) {
      return source
    }
  }

  const path = getFilePath(source, { folder, fileType })
  if (!path) {
    return undefined
  }

  const maxBytes = ["table", "data", "file"].includes(fileType)
    ? settings.DATA_MAX_BYTES
    : settings.METADATA_MAX_BYTES

  let webStream: ReadableStream<Uint8Array> | null
  if (typeof source === "string") {
    const response = await fetch(source)
    if (!response.ok) {
      throw new Error(
        `Failed to fetch ${source}: ${response.status} ${response.statusText}`,
      )
    }
    webStream = response.body
  } else {
    webStream = source.stream()
  }

  if (!webStream) {
    throw new Error("Invalid file source")
  }

  // Readable.fromWeb's types predate the global ReadableStream in Node 24
  // @ts-expect-error
  let stream = Readable.fromWeb(webStream)
  if (maxBytes) {
    stream = limitStreamSize(stream, maxBytes)
  }

  // It is an equivalent to ensureDir function that won't overwrite an existing directory
  await mkdir(nodePath.dirname(path), { recursive: true })
  // The "wx" flag ensures that the file won't overwrite an existing file
  await pipeline(stream, createWriteStream(path, { flags: "wx" }))

  return path
}

function limitStreamSize(inputStream: Readable, maxBytes: number) {
  let total = 0
  return inputStream.pipe(
    new Transform({
      transform(chunk, _encoding, callback) {
        if (total >= maxBytes) {
          inputStream.destroy()
          callback(new Error("File size exceeds the limit"))
          return
        }

        total += chunk.length
        callback(null, chunk)
      },
    }),
  )
}

function getFilePath(
  source: string | File,
  options: {
    folder: string
    fileType: FileType
  },
) {
  const { fileType } = options

  if (!source) {
    return undefined
  }

  const extension = ["table", "data", "file"].includes(fileType)
    ? getFileExtension(typeof source === "string" ? source : source.name)
    : "json"

  const fileName = `${fileType}.${extension}`
  const path = nodePath.join(options.folder, fileName)

  return path
}
