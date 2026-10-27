import type { Descriptor, RightsList } from "@fairspec/metadata"

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
