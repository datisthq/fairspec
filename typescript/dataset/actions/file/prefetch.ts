import os from "node:os"
import type { Resource } from "@fairspec/metadata"
import { getDataPaths, getIsRemotePath } from "@fairspec/metadata"
import pAll from "p-all"
import { copyFile } from "../../actions/file/copy.ts"
import { getTempFilePath } from "../../actions/file/temp.ts"

export async function prefetchFiles(resource: Resource, options?: { maxBytes?: number }) {
  const paths = getDataPaths(resource)
  if (!paths.length) return []

  const concurrency = os.cpus().length
  const newPaths = await pAll(
    paths.map(path => () => prefetchFile(path, options)),
    { concurrency },
  )

  return newPaths
}

export async function prefetchFile(path: string, options?: { maxBytes?: number }) {
  if (!getIsRemotePath(path)) {
    return path
  }

  const newPath = getTempFilePath()
  await copyFile({
    sourcePath: path,
    targetPath: newPath,
    maxBytes: options?.maxBytes,
  })

  return newPath
}
