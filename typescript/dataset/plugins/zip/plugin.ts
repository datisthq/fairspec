import type { Dataset } from "@fairspec/metadata"
import { getFileExtension } from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"
import { loadDatasetFromZip } from "./actions/dataset/load.ts"
import { saveDatasetToZip } from "./actions/dataset/save.ts"

export class ZipPlugin implements DatasetPlugin {
  async loadDataset(source: string) {
    const isZip = getIsZip(source)
    if (!isZip) return undefined

    const dataset = await loadDatasetFromZip(source)
    return dataset
  }

  async saveDataset(dataset: Dataset, options: { target: string; withRemote?: boolean }) {
    const isZip = getIsZip(options.target)
    if (!isZip) return undefined

    await saveDatasetToZip(dataset, {
      archivePath: options.target,
      withRemote: !!options?.withRemote,
    })

    return { path: undefined }
  }
}

function getIsZip(path: string) {
  return getFileExtension(path) === "zip"
}
