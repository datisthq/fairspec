import type { Dataset } from "@fairspec/metadata"
import {
  denormalizeResource,
  getBasepath,
  resolveFileDialect,
  resolveTableSchema,
  saveDescriptor,
} from "@fairspec/metadata"
import { convertDatasetToFrictionless } from "./toFrictionless.ts"

/**
 * Save a Dataset as a Frictionless Data Package (v2) descriptor
 * Paths are written relative to the descriptor; data files are not copied
 */
export async function saveDatasetToFrictionless(
  dataset: Dataset,
  options: { target: string; overwrite?: boolean },
) {
  const basepath = getBasepath(options.target)

  const resources = []
  for (const resource of dataset.resources ?? []) {
    const tableSchema = await resolveTableSchema(resource.tableSchema)
    const fileDialect = await resolveFileDialect(resource.fileDialect)
    resources.push(
      denormalizeResource({ ...resource, tableSchema, fileDialect }, { basepath }),
    )
  }

  const descriptor = convertDatasetToFrictionless({ ...dataset, resources })
  await saveDescriptor(descriptor, { path: options.target, overwrite: options.overwrite })

  return { path: options.target }
}
