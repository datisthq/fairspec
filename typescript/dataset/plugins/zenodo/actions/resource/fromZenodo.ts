import type { Resource } from "@fairspec/metadata"
import { getFileNameSlug } from "@fairspec/metadata"
import type { ZenodoFile } from "../../models/file.ts"

export function convertResourceFromZenodo(zenodoFile: ZenodoFile) {
  const path = convertPath(zenodoFile.links.self)

  const resource: Resource = {
    data: path,
    name: getFileNameSlug(zenodoFile.key) ?? zenodoFile.id,
    integrity: {
      type: "md5",
      hash: zenodoFile.checksum.replace("md5:", ""),
    },
    unstable_customMetadata: {
      zenodoKey: zenodoFile.key,
      zenodoUrl: path,
    },
  }

  return resource
}

function convertPath(link: string) {
  return link.replace("/api/", "/").replace(/\/content$/, "")
}
