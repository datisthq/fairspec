import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { FileDialect } from "../../models/fileDialect/fileDialect.ts"

export async function validateFileDialect(
  source: FileDialect | Descriptor | string,
  options?: {
    rootJsonPointer?: string
  },
) {
  const descriptor =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const $schema =
    typeof descriptor.$schema === "string"
      ? descriptor.$schema
      : `https://fairspec.org/profiles/latest/file-dialect.json`

  const profile = await loadProfile($schema, {
    profileType: "file-dialect",
  })

  const report = await validateDescriptor(descriptor, {
    profile,
    rootJsonPointer: options?.rootJsonPointer,
  })

  let fileDialect: FileDialect | undefined
  if (report.valid) {
    // Valid -> we can cast it
    fileDialect = descriptor as FileDialect
  }

  return { ...report, fileDialect }
}
