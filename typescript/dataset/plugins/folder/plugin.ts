import { stat } from "node:fs/promises"
import type { Dataset } from "@fairspec/metadata"
import { getFileExtension, getIsRemotePath } from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"
import { loadDatasetFromFolder } from "./actions/dataset/load.ts"
import { saveDatasetToFolder } from "./actions/dataset/save.ts"

export class FolderPlugin implements DatasetPlugin {
  async loadDataset(source: string) {
    const isFolder = await getIsFolder(source)
    if (!isFolder) return undefined

    const dataset = await loadDatasetFromFolder(source)
    return dataset
  }

  async saveDataset(dataset: Dataset, options: { target: string; withRemote?: boolean }) {
    const isFolderTarget = await getIsFolderTarget(options.target)
    if (!isFolderTarget) return undefined

    await saveDatasetToFolder(dataset, {
      folderPath: options.target,
      withRemote: options.withRemote,
    })

    return { path: options.target }
  }
}

async function getIsFolder(path: string) {
  const isRemote = getIsRemotePath(path)
  if (isRemote) return false

  try {
    return (await stat(path)).isDirectory()
  } catch {
    return false
  }
}

async function getIsFolderTarget(path: string) {
  if (getIsRemotePath(path)) return false
  if (await getIsFolder(path)) return true
  return !getFileExtension(path)
}
