import type { Descriptor } from "@fairspec/metadata"
import type { FrictionlessLicense } from "../../models/license.ts"

export function convertRightsFromFrictionless(licenses: FrictionlessLicense[]) {
  return licenses.map(license => {
    const rights: Descriptor = {}
    const title = license.title ?? license.name
    if (title) rights.rights = title
    if (license.path) rights.rightsUri = license.path
    if (license.name) rights.rightsIdentifier = license.name
    return rights
  })
}
