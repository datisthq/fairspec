import { createReadStream } from "node:fs"
import type { ResourceDataPath } from "@fairspec/metadata"
import { Readable, Transform } from "node:stream"
import { getIsRemotePath } from "@fairspec/metadata"

export async function loadFileStream(
  dataPath: ResourceDataPath,
  options?: {
    index?: number
    maxBytes?: number
  },
) {
  const index = options?.index ?? 0

  const paths = Array.isArray(dataPath) ? dataPath : [dataPath]
  const indexPath = paths[index]

  if (!indexPath) {
    throw new Error(`Cannot stream resource ${indexPath} at index ${index}`)
  }

  const isRemote = getIsRemotePath(indexPath)
  const stream = isRemote
    ? await loadRemoteFileStream(indexPath, options)
    : await loadLocalFileStream(indexPath, options)

  return stream
}

async function loadLocalFileStream(path: string, options?: { maxBytes?: number }) {
  const end = options?.maxBytes ? options.maxBytes - 1 : undefined
  return createReadStream(path, { end })
}

async function loadRemoteFileStream(path: string, options?: { maxBytes?: number }) {
  const response = await fetch(path)
  if (!response.body) {
    throw new Error(`Cannot stream remote resource: ${path}`)
  }

  let stream = Readable.fromWeb(response.body)

  if (options?.maxBytes) {
    stream = limitBytesStream(stream, options.maxBytes)
  }

  return stream
}

function limitBytesStream(inputStream: Readable, maxBytes: number) {
  let total = 0
  return inputStream.pipe(
    new Transform({
      transform(chunk, _encoding, callback) {
        if (total >= maxBytes) {
          this.push(null)
          callback()
          inputStream.destroy()
          return
        }

        const remaining = maxBytes - total
        if (chunk.length > remaining) {
          chunk = chunk.slice(0, remaining)
        }

        total += chunk.length
        callback(null, chunk)
      },
    }),
  )
}
