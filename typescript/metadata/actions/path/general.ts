import slugify from "@sindresorhus/slugify"
import { node } from "../../services/node.ts"

export function getIsRemotePath(path: string) {
  const protocol = getFileProtocol(path)
  return protocol !== "file"
}

export function getFileName(path: string) {
  const isRemote = getIsRemotePath(path)

  if (isRemote) {
    const pathname = new URL(path).pathname
    const fileName = pathname.split("/").slice(-1)[0]
    return fileName?.includes(".") ? fileName : undefined
  }

  if (!node) {
    throw new Error("File system is not supported in this environment")
  }

  const resolvedPath = node.path.resolve(path)
  const fileName = node.path.parse(resolvedPath).base
  return fileName?.includes(".") ? fileName : undefined
}

export function getFileNameSlug(path: string) {
  const basename = getFileBasename(path)
  if (!basename) {
    return undefined
  }

  return slugify(basename, { separator: "_" }).replace(/[^a-zA-Z0-9_]/g, "")
}

export function getFileProtocol(path: string) {
  try {
    const url = new URL(path)
    const protocolName = url.protocol.replace(":", "")

    // Handle Windows drive letters
    if (protocolName.length < 2) {
      return "file"
    }

    return protocolName
  } catch {
    return "file"
  }
}

export function getFileExtension(path: string) {
  const fileName = getFileName(path)
  const extension = fileName?.split(".").slice(-1)[0]
  return fileName !== `.${extension}` ? extension : undefined
}

export function getFileBasename(path: string) {
  const fileName = getFileName(path)
  const extension = getFileExtension(path)
  return extension ? fileName?.replace(`.${extension}`, "") : fileName
}
