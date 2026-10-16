import { mergeDatasets } from "../../../../actions/dataset/merge.ts"
import { makeCkanApiRequest } from "../../services/ckan.ts"
import type { CkanDataset } from "../../models/dataset.ts"
import { convertDatasetFromCkan } from "./fromCkan.ts"

/**
 * Load a dataset from a CKAN instance
 * @param datasetUrl URL to the CKAN dataset
 * @returns Dataset object
 */
export async function loadDatasetFromCkan(datasetUrl: string) {
  const datasetId = extractDatasetId(datasetUrl)
  if (!datasetId) {
    throw new Error(`Failed to extract dataset ID from URL: ${datasetUrl}`)
  }

  const ckanDataset = await makeCkanApiRequest<CkanDataset>({
    ckanUrl: datasetUrl,
    action: "package_show",
    payload: { id: datasetId },
  })

  for (const resource of ckanDataset.resources) {
    const resourceId = resource.id
    if (["CSV", "XLS", "XLSX"].includes(resource.format)) {
      const schema = await loadCkanSchema({ datasetUrl, resourceId })
      if (schema) {
        resource.schema = schema
      }
    }
  }

  const systemDataset = convertDatasetFromCkan(ckanDataset)
  const userDatasetPath = (systemDataset.resources ?? [])
    .filter(resource => resource.unstable_customMetadata?.ckanKey === "dataset.json")
    .map(resource => resource.unstable_customMetadata?.ckanUrl as string)
    .at(0)

  const dataset = await mergeDatasets({ systemDataset, userDatasetPath })
  dataset.resources?.forEach(resource => {
    delete resource.unstable_customMetadata
  })

  return dataset
}

/**
 * Extract dataset ID from URL
 *
 * Examples:
 * - https://hri.fi/data/en_GB/dataset/helsingin-kaupungin-verkkosivustojen-kavijaanalytiikka
 * - https://www.opendata.dk/city-of-copenhagen/parkeringszoner-information
 * - https://open.africa/dataset/pib-annual-senegal
 * - https://data.nhm.ac.uk/dataset/join-the-dots-collection-level-descriptions
 */
function extractDatasetId(datasetUrl: string) {
  const url = new URL(datasetUrl)
  const pathParts = url.pathname.split("/").filter(Boolean)
  return pathParts.at(-1)
}

/**
 * Fetch resource schema data from CKAN datastore
 */
async function loadCkanSchema(options: { datasetUrl: string; resourceId: string }) {
  try {
    // For some reason, datastore_info doesn't work
    // So we use data fetching endpoint that also returns the schema
    const result = await makeCkanApiRequest({
      ckanUrl: options.datasetUrl,
      action: "datastore_search",
      payload: { resource_id: options.resourceId, limit: 0 },
    })

    // @ts-ignore
    const fields = result.fields.filter(
      (field: any) => field.id !== "_id" && field.id !== "_full_text",
    )

    return { fields }
  } catch (error) {
    return undefined
  }
}
