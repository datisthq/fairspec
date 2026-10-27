import type { Dataset } from "@fairspec/metadata"
import { getFileName, getIsRemotePath } from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"
import { saveDatasetToFrictionless } from "./actions/dataset/save.ts"

export class FrictionlessPlugin implements DatasetPlugin {
  async saveDataset(dataset: Dataset, options: { target: string; withRemote?: boolean }) {
    if (!getIsDatapackage(options.target) || getIsRemotePath(options.target)) {
      return undefined
    }

    return await saveDatasetToFrictionless(dataset, { target: options.target })
  }
}

function getIsDatapackage(path: string) {
  return getFileName(path) === "datapackage.json"
}
