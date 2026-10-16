import { saveDescriptor } from "../../actions/descriptor/save.ts"
import { getBasepath } from "../../actions/path/basepath.ts"
import type { Dataset } from "../../models/dataset.ts"
import * as settings from "../../settings.ts"
import { denormalizeDataset } from "./denormalize.ts"

/**
 * Save a Dataset to a file path
 * Works in Node.js environments
 */
export async function saveDatasetDescriptor(
  dataset: Dataset,
  options: {
    path: string
    overwrite?: boolean
  },
) {
  const basepath = getBasepath(options.path)
  const descriptor = denormalizeDataset(dataset, { basepath })

  descriptor.$schema =
    descriptor.$schema ??
    `https://fairspec.org/profiles/${settings.FAIRSPEC_VERSION}/dataset.json`

  await saveDescriptor(descriptor, {
    path: options.path,
    overwrite: options.overwrite,
  })
}
