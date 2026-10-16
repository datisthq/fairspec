import { blob } from "node:stream/consumers"
import type { Dataset, Descriptor } from "@fairspec/metadata"
import { denormalizeDataset, stringifyDescriptor } from "@fairspec/metadata"
import { getDatasetBasepath } from "../../../../actions/dataset/basepath.ts"
import { saveResourceFiles } from "../../../../actions/resource/save.ts"
import { loadFileStream } from "../../../../actions/stream/load.ts"
import type { ZenodoRecord } from "../../models/record.ts"
import { makeZenodoApiRequest } from "../../services/zenodo.ts"
import { convertDatasetToZenodo } from "./toZenodo.ts"

export async function saveDatasetToZenodo(
  dataset: Dataset,
  options: {
    sandbox?: boolean
    apiKey: string
  },
) {
  const { apiKey, sandbox = false } = options
  const basepath = getDatasetBasepath(dataset)

  const newZenodoRecord = convertDatasetToZenodo(dataset)
  const zenodoRecord = (await makeZenodoApiRequest({
    payload: newZenodoRecord,
    endpoint: "/deposit/depositions",
    method: "POST",
    apiKey,
    sandbox,
  })) as ZenodoRecord

  const resourceDescriptors: Descriptor[] = []
  for (const resource of dataset.resources ?? []) {
    resourceDescriptors.push(
      await saveResourceFiles(resource, {
        basepath,
        withRemote: false,
        withoutFolders: true,
        saveFile: async options => {
          const upload = {
            name: options.denormalizedPath,
            data: await blob(await loadFileStream(options.normalizedPath)),
          }

          await makeZenodoApiRequest({
            endpoint: `/deposit/depositions/${zenodoRecord.id}/files`,
            method: "POST",
            upload,
            apiKey,
            sandbox,
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

  for (const denormalizedPath of ["dataset.json"]) {
    const upload = {
      name: denormalizedPath,
      data: new Blob([stringifyDescriptor(descriptor)]),
    }

    await makeZenodoApiRequest({
      endpoint: `/deposit/depositions/${zenodoRecord.id}/files`,
      method: "POST",
      upload,
      apiKey,
      sandbox,
    })
  }

  const url = new URL(zenodoRecord.links.html)
  return {
    path: `${url.origin}/records/${zenodoRecord.id}/files/dataset.json`,
    datasetUrl: `${url.origin}/uploads/${zenodoRecord.id}`,
  }
}
