import type { Dataset } from "../../models/dataset.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import { FairspecException } from "../../models/exception.ts"
import { validateDatasetDescriptor } from "./validate.ts"

/**
 * Assert a Dataset descriptor (JSON Object) against its profile
 */
export async function assertDataset(
  source: Descriptor | Dataset,
  options?: {
    basepath?: string
  },
) {
  const report = await validateDatasetDescriptor(source, options)

  if (!report.dataset) {
    throw new FairspecException(`Invalid Dataset`, { report })
  }

  return report.dataset
}
