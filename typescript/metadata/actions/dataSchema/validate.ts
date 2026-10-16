import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import type { DataSchema } from "../../models/dataSchema.ts"
import type { Descriptor } from "../../models/descriptor.ts"

export async function validateDataSchema(
  source: DataSchema | Descriptor | string,
  options?: {
    rootJsonPointer?: string
  },
) {
  const descriptor =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const $schema =
    typeof descriptor.$schema === "string"
      ? descriptor.$schema
      : `https://fairspec.org/profiles/latest/data-schema.json`

  const profile = await loadProfile($schema, {
    profileType: "data-schema",
  })

  const report = await validateDescriptor(descriptor, {
    profile,
    rootJsonPointer: options?.rootJsonPointer,
  })

  let dataSchema: DataSchema | undefined
  if (report.valid) {
    // Valid -> we can cast it
    dataSchema = descriptor as DataSchema
  }

  return { ...report, dataSchema }
}
