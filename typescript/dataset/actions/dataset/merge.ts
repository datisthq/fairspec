import type { Dataset } from "@fairspec/metadata"
import { loadDatasetDescriptor } from "@fairspec/metadata"

/**
 * Merges a custom dataset into a user dataset if provided
 */
export async function mergeDatasets(options: {
  systemDataset: Dataset
  userDatasetPath?: string
}) {
  const systemDataset = options.systemDataset

  const userDataset = options.userDatasetPath
    ? await loadDatasetDescriptor(options.userDatasetPath)
    : undefined

  return { ...systemDataset, ...userDataset }
}
