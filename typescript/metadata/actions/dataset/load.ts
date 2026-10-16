import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { resolveBasepath } from "../../actions/path/basepath.ts"
import { assertDataset } from "./assert.ts"

/**
 * Load a Dataset descriptor (JSON Object) from a file or URL
 * Ensures the descriptor is valid against its profile
 */
export async function loadDatasetDescriptor(path: string) {
  const basepath = await resolveBasepath(path)
  const descriptor = await loadDescriptor(path)
  const dataset = await assertDataset(descriptor, { basepath })
  return dataset
}
