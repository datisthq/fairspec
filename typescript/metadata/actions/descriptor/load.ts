import { node } from "../../services/node.ts"
import { getFileProtocol, getIsRemotePath } from "../../actions/path/general.ts"
import { parseDescriptor } from "./parse.ts"

/**
 * Load a descriptor (JSON Object) from a file or URL
 * Uses dynamic imports to work in both Node.js and browser environments
 * Supports HTTP, HTTPS protocols
 */
export async function loadDescriptor(
  path: string,
  options?: {
    onlyRemote?: boolean
  },
) {
  const isRemote = getIsRemotePath(path)
  if (!isRemote && options?.onlyRemote) {
    throw new Error("Cannot load descriptor for security reasons")
  }

  return !isRemote ? await loadLocalDescriptor(path) : await loadRemoteDescriptor(path)
}

async function loadLocalDescriptor(path: string) {
  if (!node) {
    throw new Error("File system is not supported in this environment")
  }

  const text = await node.fs.readFile(path, "utf-8")
  const descriptor = parseDescriptor(text)

  return descriptor
}

async function loadRemoteDescriptor(path: string) {
  const url = new URL(path)

  const protocol = getFileProtocol(path)
  if (!["http", "https"].includes(protocol)) {
    throw new Error(`Unsupported remote protocol: ${protocol}`)
  }

  const response = await fetch(url.toString())
  const descriptor = (await response.json()) as Record<string, any>

  return descriptor
}
