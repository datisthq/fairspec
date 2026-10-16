import type { Resource } from "@fairspec/metadata"
import type { ZenodoFile } from "../../models/file.ts"

export function convertResourceToZenodo(resource: Resource) {
  const zenodoFile: Partial<ZenodoFile> = {
    key: resource.name,
  }

  if (resource.integrity && resource.integrity.type === "md5") {
    zenodoFile.checksum = `md5:${resource.integrity.hash}`
  }

  return zenodoFile
}
