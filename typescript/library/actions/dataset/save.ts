import type { SaveDatasetOptions } from "@fairspec/dataset"
import type { Dataset } from "@fairspec/metadata"
import { system } from "../../system.ts"

export async function saveDataset(dataset: Dataset, options: SaveDatasetOptions) {
  for (const plugin of system.plugins) {
    const result = await plugin.saveDataset?.(dataset, options)
    if (result) {
      return result
    }
  }

  throw new Error(`No plugin can save the dataset: ${options.target}`)
}
