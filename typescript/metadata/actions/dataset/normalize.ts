import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { normalizeResource } from "../../actions/resource/normalize.ts"
import type { Dataset } from "../../models/dataset.ts"

export function normalizeDataset(
  dataset: Dataset,
  options: {
    basepath?: string
  },
) {
  dataset = copyDescriptor(dataset)

  dataset.resources = dataset.resources?.map(resource =>
    normalizeResource(resource, options),
  )

  return dataset
}
