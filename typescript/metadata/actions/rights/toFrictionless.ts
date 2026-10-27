import type { RightsList } from "../../models/datacite/rights.ts"
import type { Descriptor } from "../../models/descriptor.ts"

export function convertRightsToFrictionless(rightsList: RightsList) {
  return rightsList
    .map(rights => {
      const license: Descriptor = {}
      if (rights.rightsIdentifier) license.name = rights.rightsIdentifier
      if (rights.rightsUri) license.path = rights.rightsUri
      if (rights.rights) license.title = rights.rights
      return license
    })
    .filter(license => license.name || license.path)
}
