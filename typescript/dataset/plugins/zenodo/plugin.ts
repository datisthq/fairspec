import { getIsRemotePath } from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"
import { loadDatasetFromZenodo } from "./actions/dataset/load.ts"

export class ZenodoPlugin implements DatasetPlugin {
  async loadDataset(source: string) {
    const isZenodo = getIsZenodo(source)
    if (!isZenodo) return undefined

    const dataset = await loadDatasetFromZenodo(source)
    return dataset
  }
}

function getIsZenodo(path: string) {
  const isRemote = getIsRemotePath(path)
  if (!isRemote) return false

  return new URL(path).hostname.endsWith("zenodo.org")
}
