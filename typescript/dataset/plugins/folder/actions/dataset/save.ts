import { join } from "node:path"
import type { Dataset, Descriptor } from "@fairspec/metadata"
import { denormalizeDataset, saveDescriptor } from "@fairspec/metadata"
import { getDatasetBasepath } from "../../../../actions/dataset/basepath.ts"
import { copyFile } from "../../../../actions/file/copy.ts"
import { assertLocalPathVacant } from "../../../../actions/file/path.ts"
import { createFolder } from "../../../../actions/folder/create.ts"
import { saveResourceFiles } from "../../../../actions/resource/save.ts"

export async function saveDatasetToFolder(
  dataset: Dataset,
  options: {
    folderPath: string
    withRemote?: boolean
  },
) {
  const basepath = getDatasetBasepath(dataset)
  const { folderPath, withRemote } = options

  await assertLocalPathVacant(folderPath)
  await createFolder(folderPath)

  const resourceDescriptors: Descriptor[] = []
  for (const resource of dataset.resources ?? []) {
    resourceDescriptors.push(
      await saveResourceFiles(resource, {
        basepath,
        withRemote,
        saveFile: async options => {
          await copyFile({
            sourcePath: options.normalizedPath,
            targetPath: join(folderPath, options.denormalizedPath),
          })

          return options.denormalizedPath
        },
      }),
    )
  }

  const descriptor = {
    ...denormalizeDataset(dataset, { basepath }),
    resources: resourceDescriptors,
  }

  await saveDescriptor(descriptor, {
    path: join(folderPath, "dataset.json"),
  })

  return descriptor
}
