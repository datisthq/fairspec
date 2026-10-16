import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { denormalizeResource } from "../../actions/resource/denormalize.ts"
import type { Dataset } from "../../models/dataset.ts"

export function denormalizeDataset(
  dataset: Dataset,
  options: {
    basepath?: string
  },
) {
  dataset = copyDescriptor(dataset)

  dataset.resources = dataset.resources?.map(resource =>
    denormalizeResource(resource, options),
  )

  return dataset
}
