import type { FrictionlessRecorder } from "../../actions/frictionless/mapping.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { FrictionlessLicense } from "../../models/frictionless/license.ts"

export function convertRightsFromFrictionless(
  licenses: FrictionlessLicense[],
  options?: { record?: FrictionlessRecorder },
) {
  return licenses.map((license, index) => {
    const rights: Descriptor = {}
    const title = license.title ?? license.name
    if (title) rights.rights = title
    if (license.path) rights.rightsUri = license.path
    if (license.name) rights.rightsIdentifier = license.name

    const record = options?.record
    if (record) {
      record(`/${index}`, `/${index}`)
      record(`/${index}/rights`, license.title ? `/${index}/title` : `/${index}/name`)
      record(`/${index}/rightsUri`, `/${index}/path`)
      record(`/${index}/rightsIdentifier`, `/${index}/name`)
    }

    return rights
  })
}
