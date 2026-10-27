import { prepareFrictionlessDescriptor } from "../../actions/frictionless/convert.ts"
import { mapFrictionlessErrors } from "../../actions/frictionless/mapping.ts"
import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { inspectModel } from "../../actions/descriptor/inspect.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import { FileDialect } from "../../models/fileDialect/fileDialect.ts"
import { inspectFileDialect } from "./inspect.ts"

export async function validateFileDialect(
  source: FileDialect | Descriptor | string,
  options?: {
    rootJsonPointer?: string
  },
) {
  const loaded =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const prepared = prepareFrictionlessDescriptor(loaded, {
    kind: "fileDialect",
    rootJsonPointer: options?.rootJsonPointer,
  })
  if (!prepared.descriptor) {
    return { valid: false, errors: prepared.errors, fileDialect: undefined }
  }
  const descriptor = prepared.descriptor

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
    const parsed = inspectModel(descriptor, {
      model: FileDialect,
      rootJsonPointer: options?.rootJsonPointer,
    })
    fileDialect = parsed.data
    report.errors.push(...parsed.errors)
    if (fileDialect) {
      report.errors.push(
        ...inspectFileDialect(fileDialect, { rootJsonPointer: options?.rootJsonPointer }),
      )
    }
    report.valid = report.errors.length === 0
    if (!report.valid) fileDialect = undefined
  }

  if (prepared.mapping) {
    report.errors = mapFrictionlessErrors(report.errors, prepared.mapping)
  }

  return { ...report, fileDialect }
}
