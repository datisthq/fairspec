import { blob } from "node:stream/consumers"
import type { Dataset, Descriptor } from "@fairspec/metadata"
import {
  denormalizeDataset,
  getFileExtension,
  stringifyDescriptor,
} from "@fairspec/metadata"
import { getDatasetBasepath } from "../../../../actions/dataset/basepath.ts"
import { saveResourceFiles } from "../../../../actions/resource/save.ts"
import { loadFileStream } from "../../../../actions/stream/load.ts"
import { convertResourceToCkan } from "../../actions/resource/toCkan.ts"
import { CkanDataset } from "../../models/dataset.ts"
import { CkanResource } from "../../models/resource.ts"
import { makeCkanApiRequest } from "../../services/ckan.ts"
import { convertDatasetToCkan } from "./toCkan.ts"

export async function saveDatasetToCkan(
  dataset: Dataset,
  options: {
    apiKey: string
    ckanUrl: string
    ownerOrg: string
    datasetName: string
  },
) {
  const { apiKey, ckanUrl, ownerOrg, datasetName } = options

  const basepath = getDatasetBasepath(dataset)
  const ckanDataset = convertDatasetToCkan(dataset)

  const payload = {
    ...ckanDataset,
    name: datasetName,
    owner_org: ownerOrg,
    resources: [],
  }

  const result = await makeCkanApiRequest({
    action: "package_create",
    payload,
    ckanUrl: ckanUrl,
    apiKey: apiKey,
    schema: CkanDataset,
  })

  const url = new URL(ckanUrl)
  url.pathname = `/dataset/${result.name}`

  const resourceDescriptors: Descriptor[] = []
  for (const resource of dataset.resources ?? []) {
    resourceDescriptors.push(
      await saveResourceFiles(resource, {
        basepath,
        withRemote: true,
        withoutFolders: true,
        saveFile: async options => {
          const ckanResource = convertResourceToCkan(resource)
          const extension = getFileExtension(options.normalizedPath)

          const payload = {
            ...ckanResource,
            package_id: datasetName,
            name: options.denormalizedPath,
            format: extension ? extension.toUpperCase() : undefined,
          }

          const upload = {
            name: options.denormalizedPath,
            data: await blob(await loadFileStream(options.normalizedPath)),
          }

          const result = await makeCkanApiRequest({
            action: "resource_create",
            payload,
            upload,
            ckanUrl,
            apiKey,
            schema: CkanResource,
          })

          return result.url
        },
      }),
    )
  }

  const descriptor = {
    ...denormalizeDataset(dataset, { basepath }),
    resources: resourceDescriptors,
  }

  const descriptorResource = await makeCkanApiRequest({
    action: "resource_create",
    payload: { package_id: datasetName, name: "dataset.json" },
    upload: { name: "dataset.json", data: new Blob([stringifyDescriptor(descriptor)]) },
    ckanUrl,
    apiKey,
    schema: CkanResource,
  })

  return {
    path: descriptorResource.url,
    datasetUrl: url.toString(),
  }
}
