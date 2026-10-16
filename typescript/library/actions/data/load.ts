import type { Resource } from "@fairspec/metadata"
import {
  getSupportedFileDialect,
  getDataFirstPath,
  getDataValue,
} from "@fairspec/metadata"
import { loadDescriptor } from "@fairspec/metadata"

export async function loadData(resource: Resource) {
  const dataValue = getDataValue(resource)
  if (dataValue) return dataValue

  const firstPath = getDataFirstPath(resource)
  if (firstPath) {
    const dialect = await getSupportedFileDialect(resource, ["json"])
    if (dialect) {
      return await loadDescriptor(firstPath)
    }
  }

  return undefined
}
