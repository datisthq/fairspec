import type { Dataset } from "@fairspec/metadata"
import { system } from "../../system.ts"

export async function renderDatasetAs(dataset: Dataset, options: { format: string }) {
  for (const plugin of system.plugins) {
    const text = plugin.renderDatasetAs?.(dataset, options)
    if (text) {
      return text
    }
  }

  throw new Error(`No plugin can render the dataset as: ${options.format}`)
}
