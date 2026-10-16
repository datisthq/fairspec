import { system } from "../../system.ts"

export async function loadDataset(source: string) {
  for (const plugin of system.plugins) {
    const dataset = await plugin.loadDataset?.(source)
    if (dataset) return dataset
  }

  throw new Error(`No plugin can load the dataset: ${source}`)
}
