import { getIsRemotePath } from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"
import { loadDatasetFromGithub } from "./actions/dataset/load.ts"

export class GithubPlugin implements DatasetPlugin {
  async loadDataset(source: string) {
    const isGithub = getIsGithub(source)
    if (!isGithub) return undefined

    const dataset = await loadDatasetFromGithub(source)
    return dataset
  }
}

function getIsGithub(path: string) {
  const isRemote = getIsRemotePath(path)
  if (!isRemote) return false

  return new URL(path).hostname === "github.com"
}
