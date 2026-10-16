import { node } from "../../services/node.ts"
import { getIsRemotePath } from "./general.ts"

export async function resolveBasepath(path: string) {
  const isRemote = getIsRemotePath(path)

  // Resolves redirects
  if (isRemote) {
    const url = new URL(path)
    const response = await fetch(url.toString(), { method: "HEAD" })
    path = response.url
  }

  return getBasepath(path)
}

export function getBasepath(path: string) {
  const isRemote = getIsRemotePath(path)

  if (isRemote) {
    const normalizedPath = new URL(path).toString()
    return normalizedPath.split("/").slice(0, -1).join("/")
  }

  if (!node) {
    throw new Error("File system is not supported in this environment")
  }

  const resolvedPath = node.path.resolve(path)
  return node.path.relative(process.cwd(), node.path.parse(resolvedPath).dir)
}
