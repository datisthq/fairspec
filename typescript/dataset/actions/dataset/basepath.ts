import { join, relative, resolve, sep } from "node:path"
import {
  type Dataset,
  getBasepath,
  getDataPaths,
  getIsRemotePath,
} from "@fairspec/metadata"

export function getDatasetBasepath(dataset: Dataset) {
  const paths: string[] = []

  for (const resource of dataset.resources ?? []) {
    const resourcePaths = getDataPaths(resource)
    paths.push(...resourcePaths)
  }

  return getCommonLocalBasepath(paths)
}

export function getCommonLocalBasepath(paths: string[]) {
  const absoluteBasepaths = paths
    .filter(path => !getIsRemotePath(path))
    .map(path => resolve(getBasepath(path)))

  if (!absoluteBasepaths.length) {
    return undefined
  }

  // On Unix it split the root fs as an empty segment
  const segmentTable = absoluteBasepaths.map(path =>
    path.split(sep).map(segment => segment || "/"),
  )

  let column = 0
  const segments: string[] = []

  while (true) {
    const segmentColumn = segmentTable.map(segments => segments[column])
    const uniqueSegments = new Set(segmentColumn)

    if (uniqueSegments.size !== 1) break
    if (!segmentColumn[0]) break

    column++
    segments.push(segmentColumn[0])
  }

  if (!segments.length) {
    throw new Error("Cannot find common basepath")
  }

  const basepath = relative(process.cwd(), join(...segments))
  return basepath
}
