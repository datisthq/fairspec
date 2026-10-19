import { loadFile } from "@fairspec/dataset"
import type { Resource } from "@fairspec/metadata"
import {
  getSupportedFileDialect,
  getDataFirstPath,
  getDataValue,
} from "@fairspec/metadata"

/**
 * Load the JSON document of a resource: its inline data or its JSON file.
 * Any JSON document is accepted, including a top-level array or scalar.
 */
export async function loadData(resource: Resource) {
  const dataValue = getDataValue(resource)
  if (dataValue !== undefined) return dataValue

  const firstPath = getDataFirstPath(resource)
  if (firstPath) {
    const dialect = await getSupportedFileDialect(resource, ["json"])
    if (dialect) {
      const buffer = await loadFile(firstPath)
      const document: unknown = JSON.parse(buffer.toString("utf-8"))
      return document
    }
  }

  return undefined
}
