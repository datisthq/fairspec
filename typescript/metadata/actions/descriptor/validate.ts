import { inspectJson } from "../../actions/json/inspect.ts"
import { createReport } from "../../actions/report/create.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { Profile } from "../../models/profile.ts"

export async function validateDescriptor(
  descriptor: Descriptor,
  options: {
    profile: Profile
    rootJsonPointer?: string
  },
) {
  const errors = await inspectJson(descriptor, {
    jsonSchema: options.profile,
    rootJsonPointer: options.rootJsonPointer,
  })

  return createReport(
    errors.map(error => ({
      type: "metadata",
      ...error,
    })),
  )
}
