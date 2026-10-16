import { getIsRemotePath } from "../../actions/path/general.ts"
import type { Resource } from "../../models/resource.ts"
import { getDataPath } from "./data.ts"

export function getIsRemoteResource(resource: Resource) {
  const dataPath = getDataPath(resource)
  if (!dataPath) return false

  const paths = Array.isArray(dataPath) ? dataPath : [dataPath]
  return paths?.some(path => getIsRemotePath(path))
}
