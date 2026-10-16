import type { Dataset } from "@fairspec/metadata"
import {
  getFileExtension,
  getIsRemotePath,
  loadDatasetDescriptor,
  saveDatasetDescriptor,
} from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"

export class DescriptorPlugin implements DatasetPlugin {
  async loadDataset(source: string) {
    const isJson = getIsJson(source)
    if (!isJson) return undefined

    const dataset = await loadDatasetDescriptor(source)
    return dataset
  }

  async saveDataset(dataset: Dataset, options: { target: string; withRemote?: boolean }) {
    const isLocalJson = getIsLocalJson(options.target)
    if (!isLocalJson) return undefined

    if (!options.target.endsWith("datapackage.json")) {
      return undefined
    }

    await saveDatasetDescriptor(dataset, { path: options.target })

    return { path: options.target }
  }
}

function getIsLocalJson(path: string) {
  const isJson = getIsJson(path)
  const isRemote = getIsRemotePath(path)
  return isJson && !isRemote
}

function getIsJson(path: string) {
  const extension = getFileExtension(path)
  return extension === "json"
}
