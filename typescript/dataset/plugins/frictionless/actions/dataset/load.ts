import {
  assertDataset,
  convertDescriptorFromFrictionless,
  createReport,
  FairspecException,
  loadDescriptor,
  mapFrictionlessErrors,
  resolveBasepath,
} from "@fairspec/metadata"

/**
 * Load a descriptor as a Frictionless Data Package (v1 or v2), whatever its content
 * looks like; `loadDataset` recognises one by content without this
 */
export async function loadDatasetFromFrictionless(path: string) {
  const basepath = await resolveBasepath(path)
  const descriptor = await loadDescriptor(path)

  const converted = convertDescriptorFromFrictionless(descriptor, { kind: "dataset" })
  if (!converted.descriptor) {
    throw new FairspecException("Invalid Frictionless Data Package", {
      report: createReport(converted.errors),
    })
  }

  try {
    return await assertDataset(converted.descriptor, { basepath })
  } catch (exception) {
    if (exception instanceof FairspecException && exception.report) {
      const errors = mapFrictionlessErrors(exception.report.errors, converted.mapping)
      throw new FairspecException(exception.message, { report: createReport(errors) })
    }
    throw exception
  }
}
