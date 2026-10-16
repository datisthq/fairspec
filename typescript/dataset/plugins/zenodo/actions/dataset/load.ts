import { mergeDatasets } from "../../../../actions/dataset/merge.ts"
import { makeZenodoApiRequest } from "../../services/zenodo.ts"
import type { ZenodoRecord } from "../../models/record.ts"
import { convertDatasetFromZenodo } from "./fromZenodo.ts"

export async function loadDatasetFromZenodo(
  datasetUrl: string,
  options?: {
    apiKey?: string
  },
) {
  const { apiKey } = options ?? {}
  const sandbox = new URL(datasetUrl).host === "sandbox.zenodo.org"

  const recordId = extractRecordId(datasetUrl)
  if (!recordId) {
    throw new Error(`Failed to extract record ID from URL: ${datasetUrl}`)
  }

  const zenodoRecord = await makeZenodoApiRequest<ZenodoRecord>({
    endpoint: `/records/${recordId}`,
    apiKey,
    sandbox,
  })

  const systemDataset = convertDatasetFromZenodo(zenodoRecord)
  const userDatasetPath = (systemDataset.resources ?? [])
    .filter(resource => resource.unstable_customMetadata?.zenodoKey === "dataset.json")
    .map(resource => resource.unstable_customMetadata?.zenodoUrl as string)
    .at(0)

  const dataset = await mergeDatasets({ systemDataset, userDatasetPath })
  dataset.resources?.forEach(resource => {
    delete resource.unstable_customMetadata
  })

  return dataset
}

function extractRecordId(datasetUrl: string): string | undefined {
  const url = new URL(datasetUrl)
  const pathParts = url.pathname.split("/").filter(Boolean)
  return pathParts.at(-1)
}
