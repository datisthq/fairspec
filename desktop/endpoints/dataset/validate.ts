import { validateDataset } from "@fairspec/library"
import { temporaryDirectoryTask } from "tempy"
import { prefetchFile } from "#actions/prefetch.ts"
import { publicEndpoint } from "#endpoints/base/public.ts"
import { ValidateDatasetInput } from "#models/dataset.ts"

export const validateDatasetEndpoint = publicEndpoint
  .input(ValidateDatasetInput)
  .handler(async ({ input }) => {
    return await temporaryDirectoryTask(async folder => {
      const dataset = await prefetchFile(input.dataset, {
        folder,
        fileType: "dataset",
      })

      if (!dataset) {
        throw new Error("Dataset is required")
      }

      const report = await validateDataset(dataset)
      return report
    })
  })
