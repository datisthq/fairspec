import { join } from "node:path"
import { loadDatasetDescriptor } from "@fairspec/metadata"

export async function loadDatasetFromFolder(folderPath: string) {
  return loadDatasetDescriptor(join(folderPath, "dataset.json"))
}
